use std::os::unix::prelude::RawFd;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use nix::sys::select::{FdSet, select};
use std::os::unix::io::AsRawFd;
use socket2::{Domain, Protocol, Socket, Type, SockAddr};

pub fn start_client(tun_fd: RawFd, mtu: i32, remote_ip: Ipv4Addr) -> Result<(), String>
{
    let s = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).expect("Cannot create socket");
    let saddr = SockAddr::from(SocketAddr::new(IpAddr::V4(remote_ip), 1234));
    s.connect(&saddr).expect("Cannot connect socket");
    println!("Socket to {}:1234 created", remote_ip);

    let udp_fd = s.as_raw_fd();

    let mut buffer = vec![0; mtu as usize];

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

            let read = nix::unistd::read(tun_fd, buffer.as_mut_slice()).expect("read error");
            println!("{:02X?}", buffer);

            println!("Sending data to server");
            //s.send_to(&buffer, &saddr).expect("Cannot send data");
            let buf: [u8; 5] = [1, 2, 3, 4, 5];
            s.send_to(&buf, &saddr).expect("Cannot send data");
        }

        if set.contains(udp_fd){
            println!("Data ready to be read from udp_fd");

            let read = nix::unistd::read(tun_fd, buffer.as_mut_slice()).expect("read error");
            println!("{:02X?}", buffer);
        }
    }
}

pub fn start_server()
{

}