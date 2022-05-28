mod ping;
use crate::ping::{IcmpType,IcmpV4};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    example_ping();
}

fn example_ping(){
	let soc = ping::create_socket_client("8.8.8.8");
	let icmp = IcmpV4::create_icmp(IcmpType::EchoRequest,0,vec![1,1,45,255,190,24,11,]);
	println!("{}",icmp.to_string());
	icmp.send_ping(&soc);
	let icmp_res = IcmpV4::recv_ping(&soc);
	println!("{}",icmp_res.to_string());
}
