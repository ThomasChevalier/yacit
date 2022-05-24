mod tun;
mod ifstructs;
use std::str::FromStr;

use crate::ifstructs::IfFlags;
use crate::tun::{tun_create, if_get_flags, if_set_flags, if_set_addr};

fn main() {
    let fd = tun_create("tun0", IfFlags::IFF_TUN | IfFlags::IFF_NO_PI).expect("Cannot create interface tun0");

    let flags = if_get_flags("tun0").expect("Cannot get flags of tun0");
    if_set_flags("tun0", flags | IfFlags::IFF_UP).expect("Cannot set flags of tun0");

    let addr = std::net::Ipv4Addr::from_str("192.168.0.1").expect("Cannot parse ip address");
    if_set_addr("tun0", &addr);
}
