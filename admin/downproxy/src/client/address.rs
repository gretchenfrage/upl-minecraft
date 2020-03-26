
use regex::Regex;
use std::{
    process::{
        Command,
        Stdio,
    },
    net::{
        IpAddr,
        Ipv6Addr,
    },
};

pub fn local_client_address() -> Result<IpAddr, ()> {
    let cmd_str = "ip -oneline -family inet6 address show enp8s0";
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
    let line = stdout.lines()
        .find(|line| line.contains("scope global dynamic noprefixroute"))
        .map(String::from)
        .ok_or_else(||
            error!("no line of {:?} contained {:?}",
                stdout,
                "scope global dynamic noprefixroute"))?;

    let regex = r##"^\d+:\s*enp8s0\s+inet6\s+(?P<address>([0-9]|[a-z]|:)+)/\d+\s+"##;
    let regex = Regex::new(regex).unwrap();
    let group = regex.captures(&line)
        .ok_or_else(|| error!("regex didn't match {:?}", line))?
        .name("address")
        .unwrap()
        .as_str()
        .to_owned();

    let addr = group.parse::<Ipv6Addr>()
        .map_err(|e| error!("failed to parse Ipv6Addr from {:?}: {}", group, e))?;

    Ok(IpAddr::V6(addr))
}