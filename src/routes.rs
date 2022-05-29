use std::{os::unix::prelude::RawFd, io::Cursor, net::Ipv4Addr};

use nix::sys::socket::{socket, AddressFamily, SockType, SockFlag, SockProtocol};

use neli::{
    consts::{nl::*, rtnl::*},
    err::NlError,
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

fn create_default_route(iface_name: &String) -> Result<(), String>{
    let sock = open_netlink()?;

    let mut rtmsg = Rtmsg {
        rtm_family: RtAddrFamily::Inet,
        rtm_dst_len: 0,
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
        .map_err(|err| format!("Cannot create route netlink attribute: {}", err))?;
    rtmsg.rtattrs.push(attr1);

    let nlhdr = {
        let len = None;
        let nl_type = Rtm::Newroute;
        let flags = NlmFFlags::new(&[NlmF::Create, NlmF::Excl, NlmF::Request]);
        let seq = None;
        let pid = None;
        let payload = rtmsg;
        Nlmsghdr::new(len, nl_type, flags, seq, pid, NlPayload::Payload(payload))
    };

    let mut buffer = Cursor::new(Vec::new());
    nlhdr.to_bytes(&mut buffer).map_err(|err| format!("Cannot convert netlink messages to bytes: {}", err))?;

    nix::sys::socket::send(sock, buffer.get_ref(), nix::sys::socket::MsgFlags::empty())
        .map_err(|err| format!("Cannot send netlink message: {}", err))?;

    Ok(())
}

pub fn create_route(iface_name: &String, remote_ip: &Ipv4Addr) -> Result<(), String>{
    create_default_route(iface_name)
    // create_remote_route()
}