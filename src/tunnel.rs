use std::os::unix::prelude::RawFd;
use std::os::unix::io::AsRawFd;
use std::net::Ipv4Addr;
use nix::poll::{poll, PollFd, PollFlags};

use super::ping;

pub fn start_client(tun_fd: RawFd, mtu: i32, remote_ip: Ipv4Addr) -> Result<(), String>
{
    let client_sock = ping::create_socket_client(remote_ip)?;
    println!("Icmp socket to {} created", remote_ip);

    let soc = ping::create_socket_server()?;
    let icmp_fd: RawFd = soc.as_raw_fd();

    let mut buffer: Vec<u8> = vec![0; mtu as usize];

    let mut poll_fd = [
        PollFd::new(tun_fd, PollFlags::POLLIN),
        PollFd::new(icmp_fd, PollFlags::POLLIN)
        ];

    loop{
        println!("Waiting for data");
        poll(&mut poll_fd, -1)
            .map_err(|err| format!("poll returned an error: {}", err))?;

        let tun_flags = poll_fd[0].revents()
            .ok_or_else(|| format!("Kernel provided unknown status flag for poll revents"))?;

        if !(tun_flags & PollFlags::POLLIN).is_empty(){
            let read = nix::unistd::read(tun_fd, buffer.as_mut_slice())
                .map_err(|err| format!("[tun] read error: {}", err))?;

            let mut payload = buffer.clone();
            payload.truncate(read);

            println!("[tun] Read {} bytes", payload.len());
            println!("[tun] Sending ICMP packet");
            let icmp_packet = ping::IcmpV4::create_icmp(ping::IcmpType::EchoRequest, 0, payload);
            icmp_packet.send_ping(&client_sock)?;
        }

        // Reception des pings du serveur
        let icmp_flags = poll_fd[1].revents()
            .ok_or_else(|| format!("Kernel provided unknown status flag for poll revents"))?;

        if !(icmp_flags & PollFlags::POLLIN).is_empty(){
            let (icmp_res, _) = ping::IcmpV4::recv_ping(&soc, mtu);
            println!("[icmp] Read payload of {} bytes",icmp_res.payload.len());

            if icmp_res.is_request() {
                println!("[icmp] Sending it to tun interface");
                nix::unistd::write(tun_fd, icmp_res.payload.as_slice())
                    .map_err(|err| format!("write error {}", err))?;
            }
        }
    }
}

pub fn start_server(tun_fd: RawFd, mtu: i32) -> Result<(), String>
{
    let soc = ping::create_socket_server()?;
    println!("Icmp socket created");
    let icmp_fd: RawFd = soc.as_raw_fd();

    let mut client_sock: Option<socket2::Socket> = None;

    let mut buffer: Vec<u8> = vec![0; mtu as usize];

    let mut poll_fd = [
        PollFd::new(tun_fd, PollFlags::POLLIN),
        PollFd::new(icmp_fd, PollFlags::POLLIN)
        ];

    loop {
        println!("---");
        poll(&mut poll_fd, -1)
            .map_err(|err| format!("poll returned an error: {}", err))?;


        // Reception des pings du client
        let icmp_flags = poll_fd[1].revents()
            .ok_or_else(|| format!("Kernel provided unknown status flag for poll revents"))?;

        if !(icmp_flags & PollFlags::POLLIN).is_empty(){

            let (icmp_res, client_addr) = ping::IcmpV4::recv_ping(&soc, mtu);
            println!("[icmp] Read payload of {} bytes",icmp_res.payload.len());

            if icmp_res.is_request() {
                if client_sock.is_none() {
                    let sock_v4 = client_addr.as_socket_ipv4()
                        .ok_or_else(|| format!("Cannot cast socket to ipv4"))?;
                    
                    client_sock = Some(ping::create_socket_client(*sock_v4.ip())?);
                    println!("[icmp] Created client socket (ip: {})", sock_v4.ip());
                }
    
                println!("[icmp] Sending it to tun interface");
                nix::unistd::write(tun_fd, icmp_res.payload.as_slice())
                    .map_err(|err| format!("write error {}", err))?;
            }
        }

        // Reception des reponses du reseau
        let tun_flags = poll_fd[0].revents()
            .ok_or_else(|| format!("Kernel provided unknown status flag for poll revents"))?;

        if !(tun_flags & PollFlags::POLLIN).is_empty(){
            let read = nix::unistd::read(tun_fd, buffer.as_mut_slice())
                .map_err(|err| format!("[tun] read error: {}", err))?;

            let mut payload = buffer.clone();
            payload.truncate(read);

            println!("[tun] Read {} bytes", payload.len());
            println!("[tun] Sending ICMP packet");

            let icmp_packet = ping::IcmpV4::create_icmp(ping::IcmpType::EchoRequest, 0, payload);
            match &client_sock {
                Some(s) => icmp_packet.send_ping(s)?,
                None => return Err("Cannot send data to client without initial connection".to_string())
            }
        }
    }
}