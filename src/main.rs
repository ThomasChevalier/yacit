mod tun;
mod ifstructs;
use std::str::FromStr;
use nix::sys::select::{FdSet, select};

use crate::ifstructs::IfFlags;
use crate::tun::*;

const MTU: i32 = 1500;

fn configure_interface() -> i32 {
    let tun_fd = tun_create("tun0", IfFlags::IFF_TUN | IfFlags::IFF_NO_PI).expect("Cannot create interface tun0");

    let flags = if_get_flags("tun0").expect("Cannot get flags of tun0");
    if_set_flags("tun0", flags | IfFlags::IFF_UP).expect("Cannot set flags of tun0");

    if_set_mtu("tun0", MTU).expect("Cannot set MTU of tun0");

    let addr = std::net::Ipv4Addr::from_str("192.168.0.1").expect("Cannot parse ip address");
    let mask = std::net::Ipv4Addr::from_str("255.255.0.0").expect("Cannot parse netmask");
    if_set_addr("tun0", &addr, &mask).expect("Cannot set interface ip address");

    tun_fd
}

fn read_data(tun_fd: i32) {
    loop{
        let mut set = FdSet::new();
        set.insert(tun_fd);
        let fd_max = set.highest().expect("FdSet should have a maximum element");
        
        // Todo: use poll
        println!("Waiting for some data to read");
        let total_ready = select(fd_max+1, &mut set, None, None, None).expect("select error");

        if set.contains(tun_fd){
            println!("Data ready to be read from tun0");
            let mut buffer: [u8; MTU as usize] = [0; MTU as usize];

            let read = nix::unistd::read(tun_fd, &mut buffer).expect("read error");
            println!("{:02X?}", buffer);
        }
    }
}

fn main() {
    let tun_fd = configure_interface();
    read_data(tun_fd);
}
