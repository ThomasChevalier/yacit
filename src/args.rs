use clap::{Parser, ArgGroup};
use std::net::Ipv4Addr;
use std::str::FromStr;

// Test if a string is a valid network interface name
fn iface_valid(s: &str) -> Result<String, String> {
    let name: String = s.parse().map_err(|_| "`{}` cannot be parsed to String")?;
    if name.contains(" "){
        Err(format!("Interface name cannot contain space"))
    }
    else if name.len() > libc::IFNAMSIZ {
        Err(format!(
            "Interface name cannot exceed {} characters",
            libc::IFNAMSIZ
        ))
    }
    else{
        Ok(name)
    }
}

// Test if a string is a valid IPv4
fn ipv4_valid(s: &str) -> Result<Ipv4Addr, String> {
    Ok(Ipv4Addr::from_str(s).map_err(|_| "`{}` is not a valid Ipv4")?)
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("client-or-server")
        .required(true)
        .args(&["server", "remote-ip"]),
))]
pub struct Args {
    /// Act as a server
    #[clap(short, long)]
    pub server: bool,

    /// Name of the yacit network interface
    #[clap(short='n', long, default_value_t = String::from("yacit0"), parse(try_from_str=iface_valid))]
    pub iface_name: String,

    /// Name of the network interface from where the VPN traffic should exit
    #[clap(short, long, parse(try_from_str=iface_valid))]
    pub out_iface_name: Option<String>,

    /// IP address of the yacit server
    #[clap(short, long, parse(try_from_str=ipv4_valid), requires="out-iface-name")]
    pub remote_ip: Option<Ipv4Addr>,

    /// Internal IP address for the interface
    #[clap(long, default_value_t = Ipv4Addr::from_str("10.0.0.2").unwrap(), parse(try_from_str=ipv4_valid))]
    pub internal_ip: Ipv4Addr,

    /// Internal netmask for the interface
    #[clap(long, default_value_t = Ipv4Addr::from_str("255.255.0.0").unwrap(), parse(try_from_str=ipv4_valid))]
    pub internal_netmask: Ipv4Addr,

    /// MTU
    #[clap(short, long)]
    pub mtu: i32,
}