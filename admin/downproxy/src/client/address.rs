
use std::{
    process::{
        Command,
        Stdio,
    },
    net::{
        IpAddr,
        Ipv4Addr,
    },
};

pub fn local_client_address() -> Result<IpAddr, ()> {
    let cmd_str = "curl ifconfig.me";

    let mut cmd_parts = cmd_str.split_whitespace();
    let output = Command::new(cmd_parts.next().unwrap())
        .args(cmd_parts)
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| 
            error!("error executing ip command: {}", e))?;
    if !output.status.success() {
        return Err({});
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| 
            error!("ip command output not utf-8: {}", e))?;

    let addr = stdout.parse::<Ipv4Addr>()
        .map_err(|e| error!("failed to parse Ipv6Addr from {:?}: {}", stdout, e))?;

    Ok(IpAddr::V4(addr))
}