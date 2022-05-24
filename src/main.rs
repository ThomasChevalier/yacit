mod tun;
use crate::tun::{tun_create, IFF_TUN, IFF_NO_PI};

fn main() {
    tun_create("tun0", IFF_TUN | IFF_NO_PI).expect("Cannot create interface tun0");
}
