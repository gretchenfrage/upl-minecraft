
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate yup_oauth2;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate reqwest;
extern crate num_cpus;

pub mod client;
pub mod gcs;
pub mod server;

use std::{
    process::exit,
    env,
    time::Duration,
    process,
    fs,
};
use gcs::{GcsAccess, GcsClient};
use server::PortPair;
use tokio::time::delay_for;

macro_rules! argv_branch {
    ($argv:expr, {
        $( $function:ident ),* $(,)?
    })=>{{
        fn f<V>(v: V) -> impl Iterator<Item=String>
        where
            V: IntoIterator<Item=String>
        {
            v.into_iter()
        }

        let mut argi = f($argv);
        match argi.next() {
            Some(subcmd) => {
                let tail: Vec<String> = argi.collect();
                match subcmd.as_str() {
                    $(
                    stringify!($function) => {
                        ($function)(tail).await;
                        true
                    },
                    )*
                    _ => false
                }
            },
            None => false,
        }
    }};
}

pub static TOKEN_PATH_VAR: &str = "TOKEN_PATH";
pub static BUCKET: &str = "mcupl-var";
pub static OBJECT: &str = "host-address_2";

async fn local_client_address(_args: Vec<String>) {
    let addr = client::address::local_client_address()
        .unwrap_or_else(|()| exit(1));
    println!("{}", addr.to_string());
}

async fn download_host_address(_args: Vec<String>) {
    let addr = server::download_host_address().await
        .unwrap_or_else(|()| exit(1));
    println!("{}", addr.to_string());
}

async fn test_token(args: Vec<String>) {
    let path = args.get(0)
        .unwrap_or_else(|| {
            error!("required argument TOKEN_PATH missing");
            exit(1);
        });

    let auth = GcsAccess::new_from_path(path).await
        .unwrap_or_else(|()| exit(1));
    let token = auth.token().await
        .unwrap_or_else(|()| exit(1));
    println!("{}", token);
}

async fn upload_host_address(_args: Vec<String>) {
    info!("getting local client address");
    let addr = client::address::local_client_address()
        .unwrap_or_else(|()| exit(1));
    info!("address = {}", addr);

    info!("looking for token path in env var {}", TOKEN_PATH_VAR);
    let auth = GcsAccess::new_from_env_path(TOKEN_PATH_VAR).await
        .unwrap_or_else(|()| exit(1));
    let gcs = GcsClient::new(auth);
    info!("constructing gcs client");

    info!("posting object");
    let body = addr.to_string().into_bytes();
    gcs.set(BUCKET, OBJECT, body).await
        .unwrap_or_else(|()| exit(1));
    info!("success!");
}

async fn maintain_host_address_object(_args: Vec<String>) -> ! {
    let wait_period = Duration::from_secs(4);

    info!("looking for token path in env var {}", TOKEN_PATH_VAR);
    let auth = GcsAccess::new_from_env_path(TOKEN_PATH_VAR).await
        .unwrap_or_else(|()| exit(1));
    info!("constructing gcs client");
    let gcs = GcsClient::new(auth);

    let mut curr_online = server::download_host_address().await.ok();
    trace!("current GCS host address = {:?}", curr_online);
    loop {
        let curr = client::address::local_client_address().ok();
        trace!("current actual host address = {:?}", curr);
        let curr = match curr {
            Some(addr) => addr,
            None => {
                info!("failed to determine local address");
                continue;
            }
        };

        if Some(curr) != curr_online {
            info!("host address changed from {:?} to {:?}", curr_online, curr);
            info!("updating gcs object");

            let body = curr.to_string().into_bytes();
            if gcs.set(BUCKET, OBJECT, body).await.is_ok() {
                info!("success!");
                curr_online = Some(curr);
            }
        }

        delay_for(wait_period).await;
    }
}

async fn run_server(args: Vec<String>) {
    if args.len() == 0 {
        error!("no ports given");
        exit(1);
    }

    let ports = args.iter()
        .map(|s| s.parse::<PortPair>())
        .collect::<Result<Vec<PortPair>, _>>()
        .unwrap_or_else(|()| exit(1));
    server::run(ports).await
        .unwrap_or_else(|()| exit(1));
}

static MAN: &str = r####"
Special proxy software for upl-minecraft.

USAGE:
    downproxy [SUBCOMMAND] [ARGS]

ENVIRONMENT VARIABLES:
    TOKEN_PATH

        Path to a JSON file for the service account to 
        access google cloud storage. Requires by commands
        which write to GCS.

    RUST_LOG

        Log-level in the de facto standard format
        established by env_logger.

    WRITE_OWN_PID_TO

        An optional path. Upon command startup, the process
        will write its own process ID to a file at this
        path.

SUBCOMMANDS:
    help

        Print this page.

    local_client_address

        Print the host's current IPV6 address.
        (Very platform-detail specific)

    test_token [CREDENTIAL_PATH]

        Print out a GCS authorization token for the 
        service account.

    upload_host_address

        Query the local client address and write it to the 
        GCS object.

    download_host_address

        Download and print the host address from the GCS 
        object.

    run_server [PORT_PAIR ...]

        Start up a full proxy server. The arguments are 
        a list of ports to bind.

        The format of a port pair is either:
        - A single u16.
          (examples: "25565", "9999", "80")
        - An internal (server), external (client) u16 
          tuple, separated by a colon.
          (examples: "1000:2000", "5002:25565")

    maintain_host_address_object

        Repeatedly query the local client address, 
        and update the GCS host address object if it 
        every becomes inconsistent.

"####;

async fn help(_args: Vec<String>) {
    println!("{}", MAN);
}

fn write_own_pid() {
    if let Ok(path) = env::var("WRITE_OWN_PID_TO") {
        info!("writing own PID to {:?}", path);
        let pid = process::id();
        info!("own pid = {}", pid);
        
        fs::write(path, pid.to_string())
            .map_err(|e| error!("error writing own pid: {}", e))
            .unwrap_or_else(|()| exit(1));
    }
}


fn main() {
    env_logger::init();

    use tokio::runtime::Builder;
    let mut runtime = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(num_cpus::get().max(4))
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();
    runtime.block_on(async move {
        write_own_pid();

        if !argv_branch!(env::args().skip(1), {
            local_client_address,
            test_token,
            upload_host_address,
            download_host_address,
            run_server,
            maintain_host_address_object,
            help,
        }) {
            error!("invalid usage");
            println!("{}", MAN);
            exit(1);
        }
    });
}
