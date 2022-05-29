mod tun;
mod ifstructs;
mod args;

use std::{str::FromStr, os::unix::prelude::AsRawFd};
use nix::sys::select::{FdSet, select};

use socket2::{Domain, Protocol, Socket, Type, SockAddr};
use tun::create_tun_interface;
use std::net::{SocketAddr, IpAddr};

use clap::Parser;


const MTU: i32 = 1400;


fn read_data(tun_fd: i32) {

    let s = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).expect("Cannot create socket");
    let addr = std::net::Ipv4Addr::from_str("10.0.0.254").expect("Cannot parse ip address");
    let saddr = SockAddr::from(SocketAddr::new(IpAddr::V4(addr), 1234));
    s.connect(&saddr).expect("Cannot connect socket");
    println!("Socket to 10.0.0.254:1234 created");

    let udp_fd = s.as_raw_fd();

    loop{
        let mut set = FdSet::new();
        set.insert(tun_fd);
        set.insert(udp_fd);
        let fd_max = set.highest().expect("FdSet should have a maximum element");
        
        // Todo: use poll
        println!("Waiting for some data to read");
        let total_ready = select(fd_max+1, &mut set, None, None, None).expect("select error");

        if set.contains(tun_fd){
            println!("Data ready to be read from tun_fd");
            let mut buffer: [u8; MTU as usize] = [0; MTU as usize];

            let read = nix::unistd::read(tun_fd, &mut buffer).expect("read error");
            println!("{:02X?}", buffer);

            println!("Sending data to server");
            //s.send_to(&buffer, &saddr).expect("Cannot send data");
            let buf: [u8; 5] = [1, 2, 3, 4, 5];
            s.send_to(&buf, &saddr).expect("Cannot send data");
        }

        if set.contains(udp_fd){
            println!("Data ready to be read from udp_fd");
            let mut buffer: [u8; MTU as usize] = [0; MTU as usize];

            let read = nix::unistd::read(tun_fd, &mut buffer).expect("read error");
            println!("{:02X?}", buffer);
        }
    }
}

fn try_main() -> Result<(), String> {
    let args = args::Args::parse();

    let tun_fd = create_tun_interface(args.iface_name, args.internal_ip, args.internal_netmask, args.mtu)?;
    read_data(tun_fd);

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }
}
