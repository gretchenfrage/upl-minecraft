
use crate::{
    BUCKET,
    OBJECT,
    gcs::public::gcs_get_string,
};
use std::{
    net::IpAddr,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
    fmt::Debug,
};
use tokio::{
    prelude::*,
    time::delay_for,
    net::{
        TcpListener,
        TcpStream,
        ToSocketAddrs,
    },
    io::{AsyncRead, AsyncWrite},
};
use regex::Regex;


pub async fn download_host_address() -> Result<IpAddr, ()> {
    let string = gcs_get_string(BUCKET, OBJECT).await?;

    IpAddr::from_str(&string)
        .map_err(|e| 
            error!("GCS response not parseable as IpAddr: {}", e))
}

#[derive(Clone)]
pub struct CurrentHostAddress {
    cell: Arc<Mutex<Option<IpAddr>>>
}

impl CurrentHostAddress {
    pub fn new() -> CurrentHostAddress {
        let handle_0 = CurrentHostAddress {
            cell: Arc::new(Mutex::new(None))
        };
        let handle_1 = handle_0.clone();

        let poll_period = Duration::from_secs(4);

        tokio::spawn(async move {
            let handle = handle_0;
            let mut addr_cached = None;
            loop {
                trace!("checking if host address changed");
                let addr_gcs = download_host_address().await.ok();
                
                if addr_gcs.is_none() {
                    continue;
                }
                if addr_gcs != addr_cached {
                    info!("host address changed from {:?} to {:?}",
                        addr_cached,
                        addr_gcs);

                    handle.set(addr_gcs);
                    addr_cached = addr_gcs;
                } else {
                    trace!("host address is the same");
                }

                delay_for(poll_period).await;
            }
        });

        handle_1
    }

    pub fn get(&self) -> Option<IpAddr> {
        let guard = self.cell.lock().unwrap();
        Clone::clone(&*guard)
    }

    pub fn set(&self, value: Option<IpAddr>) {
        let mut guard = self.cell.lock().unwrap();
        *guard = value;
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PortPair {
    pub internal: u16,
    pub external: u16,
}

impl From<(u16, u16)> for PortPair {
    fn from((internal, external): (u16, u16)) -> Self {
        PortPair { internal, external }
    }
}

impl From<u16> for PortPair {
    fn from(port: u16) -> Self {
        PortPair { 
            internal: port,
            external: port,
        }
    }
}

impl FromStr for PortPair {
    type Err = ();
    fn from_str(s: &str) -> Result<PortPair, ()> {
        if let Ok(port) = s.parse::<u16>() {
            return Ok(PortPair::from(port));
        }

        let regex = r##"(?P<internal>\d+):(?P<external>\d+)"##;
        let regex = Regex::new(regex).unwrap();

        if let Some(caps) = regex.captures(s) {
            let internal = caps.name("internal").unwrap()
                .as_str()
                .parse::<u16>().unwrap();
            let external = caps.name("external").unwrap()
                .as_str()
                .parse::<u16>().unwrap();
            
            return Ok(PortPair::from((internal, external)));
        }

        error!("not a valid port pair: {:?}", s);
        Err({})
    }
}

pub async fn run<P>(ports: P) -> Result<(), ()>
where
    P: IntoIterator,
    <P as IntoIterator>::Item: Into<PortPair>
{
    let curr_host_addr = CurrentHostAddress::new();

    for port in ports {
        let PortPair {
            internal,
            external,
        } = port.into();

        let addr = ("127.0.0.1", external);
        let mut listener = TcpListener::bind(addr).await
            .map_err(|e| 
                error!("failed to bind to port {}: {}", external, e))?;

        let curr_host_addr = curr_host_addr.clone();
        tokio::spawn(async move {
            loop {
                if let Some(host_addr) = curr_host_addr.get() 
                {
                    let proxy_to = (host_addr, internal);
                    let _ = try_open_proxy(&mut listener, proxy_to).await;
                }
            }
        });
    }

    loop {
        delay_for(Duration::from_secs(10000)).await;
    }
}

async fn try_open_proxy<A>(
    listener: &mut TcpListener,
    proxy_to: A,
) -> Result<(), ()>
where
    A: ToSocketAddrs + Debug
{
    
    // accept the incoming socket
    let (socket_a, origin) = listener.accept().await
        .map_err(|e|
            error!("failed to accept connection: {}", e))?;

    info!("opening proxy from {:?} to {:?}", origin, proxy_to);

    // open the outgoing socket
    let socket_b = TcpStream::connect(proxy_to).await
        .map_err(|e|
            error!("failed to open proxy connection: {}", e))?;

    // cross-connect the sockets
    let (ar, aw) = tokio::io::split(socket_a);
    let (br, bw) = tokio::io::split(socket_b);
    spawn_pipe(ar, bw, origin);
    spawn_pipe(br, aw, origin);

    // success
    Ok({})
}

fn spawn_pipe<R, W, O>(mut read: R, mut write: W, origin: O)
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
    O: Debug + Send + Unpin + 'static,
{
    tokio::spawn(async move {
        let mut buf = Box::new([0; 1024]);
        loop {
            let n = match read.read(&mut *buf).await {
                Ok(0) => {
                    info!("ending proxy from {:?}", origin);
                    break;
                },
                Ok(n) => n,
                Err(e) => {
                    error!("proxy read failure: {}", e);
                    break;
                },
            };

            if let Err(e) = write.write_all(&buf[..n]).await {
                error!("proxy write failure: {}", e);
                break;
            }
        }
    });
}