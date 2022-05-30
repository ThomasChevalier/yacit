use std::{os::unix::prelude::RawFd, io::Cursor, net::Ipv4Addr, str::FromStr};

use nix::sys::socket::{socket, send, AddressFamily, SockType, SockFlag, SockProtocol, MsgFlags};

use neli::{
    consts::{nl::*, rtnl::*},
    nl::{NlPayload, Nlmsghdr},
    rtnl::*,
    types::RtBuffer,
    ToBytes
};


fn open_netlink() -> Result<RawFd, String> {
    let fd = socket(AddressFamily::Netlink, SockType::Raw,
        SockFlag::empty(), Some(SockProtocol::NetlinkRoute))
        .map_err(|err| format!("creation of netlink socket failed: {}", err))?;
    
    Ok(fd)
}

fn create_generic_route(iface_name: &String, dst: &Ipv4Addr, dst_len: u8) -> Result<Nlmsghdr<Rtm, Rtmsg>, String> {
    let mut rtmsg = Rtmsg {
        rtm_family: RtAddrFamily::Inet,
        rtm_dst_len: dst_len,
        rtm_src_len: 0,
        rtm_tos: 0,
        rtm_table: RtTable::Main,
        rtm_protocol: Rtprot::Boot,
        rtm_scope: RtScope::Link,
        rtm_type: Rtn::Unicast,
        rtm_flags: RtmFFlags::empty(),
        rtattrs: RtBuffer::new(),
    };

    let if_idx = nix::net::if_::if_nametoindex(iface_name.as_str())
        .map_err(|err| format!("Cannot get interface index of {}: {}", iface_name, err))?;

    let attr1 = Rtattr::new(None, Rta::Oif, if_idx)
        .map_err(|err| format!("Cannot create route netlink attribute (Oif): {}", err))?;
    rtmsg.rtattrs.push(attr1);

    let attr2 = Rtattr::new(None, Rta::Dst, dst.octets().to_vec())
    .map_err(|err| format!("Cannot create route netlink attribute (Dst): {}", err))?;
    rtmsg.rtattrs.push(attr2);

    let nlhdr = {
        let len = None;
        let nl_type = Rtm::Newroute;
        let flags = NlmFFlags::new(&[NlmF::Create, NlmF::Excl, NlmF::Request]);
        let seq = None;
        let pid = None;
        let payload = rtmsg;
        Nlmsghdr::new(len, nl_type, flags, seq, pid, NlPayload::Payload(payload))
    };

    Ok(nlhdr)
}

fn create_and_send(sock: RawFd, iface_name: &String, dst: &Ipv4Addr, dst_len: u8) -> Result<(), String> {
    let msg = create_generic_route(iface_name, dst, dst_len)?;

    let mut buffer = Cursor::new(Vec::new());
    msg.to_bytes(&mut buffer).map_err(|err| format!("Cannot convert netlink messages to bytes: {}", err))?;

    send(sock, buffer.get_ref(), MsgFlags::empty())
        .map_err(|err| format!("Cannot send netlink message: {}", err))?;

    Ok(())
}

pub fn create_route(iface_name: &String, out_iface_name: &String, remote_ip: &Ipv4Addr) -> Result<(), String>{
    let sock = open_netlink()?;

    create_and_send(sock, iface_name, &Ipv4Addr::from_str("0.0.0.0").unwrap(), 1)?;
    create_and_send(sock, iface_name, &Ipv4Addr::from_str("128.0.0.0").unwrap(), 1)?;
    create_and_send(sock, out_iface_name, remote_ip, 32)?;
    Ok(())
}

pub fn enable_ip_forward(enabled: bool) -> Result<(), String> {
    let fd: RawFd = nix::fcntl::open("/proc/sys/net/ipv4/ip_forward", nix::fcntl::OFlag::O_RDWR, nix::sys::stat::Mode::empty())
    .map_err(|err| format!("Cannot open special ip_forward file: {}", err))?;
    if enabled {
        nix::unistd::write(fd, "1\n".as_bytes())
            .map_err(|err| format!("Cannot enable ip forwarding: {}", err))?;
    }else{
        nix::unistd::write(fd, "0\n".as_bytes())
        .map_err(|err| format!("Cannot enable ip forwarding: {}", err))?;
    }
    Ok(())
}