mod tun;
mod ifstructs;
mod args;
mod tunnel;
mod ping;

use crate::ping::{IcmpType,IcmpV4};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use clap::Parser;

fn try_main() -> Result<(), String> {
    let args = args::Args::parse();

    example_ping();

    let tun_fd = tun::create_tun_interface(args.iface_name, args.internal_ip, args.internal_netmask, args.mtu)?;

    if args.server {
        // tunnel::start_server()?;
    } else {
        let remote = args.remote_ip.ok_or_else(|| format!("No remote ip but one expected"))?;
        tunnel::start_client(tun_fd, args.mtu, remote)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }  
}

fn example_ping(){
	let soc = ping::create_socket_client("8.8.8.8");
	let icmp = IcmpV4::create_icmp(IcmpType::EchoRequest,0,vec![1,1,45,255,190,24,11,]);
	println!("{}",icmp.to_string());
	icmp.send_ping(&soc);
	let icmp_res = IcmpV4::recv_ping(&soc);
	println!("{}",icmp_res.to_string());
}
