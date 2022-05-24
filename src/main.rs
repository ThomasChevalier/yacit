mod tun;
use crate::tun::{tun_create, IFF_TUN, IFF_NO_PI, IFF_UP, if_get_flags, if_set_flags};

fn main() {
    let fd = tun_create("tun0", IFF_TUN | IFF_NO_PI).expect("Cannot create interface tun0");

    let flags = if_get_flags("tun0").expect("Cannot get flags of tun0");
    if_set_flags("tun0", flags | IFF_UP).expect("Cannot set flags of tun0");
}
