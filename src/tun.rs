use nix;
use crate::ifstructs::{ifreq, IfFlags};
use libc;
use std::os::unix::prelude::RawFd;
use std::net::Ipv4Addr;

mod ioctl {
    use crate::ifstructs::ifreq;
    use std::os::unix::prelude::RawFd;

    // linux/include/uapi/linux/if_tun.h
    const TUN_IOC_MAGIC: u8 = 'T' as u8;
    const TUN_IOC_SET_IFF: u8 = 202;
    const TUN_IOC_SET_PERSIST: u8 = 203;

    pub unsafe fn tun_set_iff(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, nix::request_code_write!(TUN_IOC_MAGIC, TUN_IOC_SET_IFF, std::mem::size_of::<i32>()), data);
        nix::errno::Errno::result(res)
    }
    
    pub unsafe fn tun_set_persist(fd: RawFd, data: *const bool) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, nix::request_code_write!(TUN_IOC_MAGIC, TUN_IOC_SET_PERSIST, std::mem::size_of::<i32>()), data);
        nix::errno::Errno::result(res)
    }
    
    pub unsafe fn if_get_flags(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, libc::SIOCGIFFLAGS, data);
        nix::errno::Errno::result(res)
    }
    
    pub unsafe fn if_set_flags(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, libc::SIOCSIFFLAGS, data);
        nix::errno::Errno::result(res)
    }

    pub unsafe fn if_set_mtu(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, libc::SIOCSIFMTU, data);
        nix::errno::Errno::result(res)
    }

    pub unsafe fn if_set_addr(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, libc::SIOCSIFADDR, data);
        nix::errno::Errno::result(res)
    }

    pub unsafe fn if_set_netmask(fd: RawFd, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
        let res = libc::ioctl(fd, libc::SIOCSIFNETMASK, data);
        nix::errno::Errno::result(res)
    }
}

fn get_dummy_socket() -> Result<RawFd, nix::errno::Errno> {
    use nix::sys::socket::*;
    socket(AddressFamily::Inet, SockType::Datagram, SockFlag::empty(), None)
}

pub fn tun_create(name: &str, flags: IfFlags) -> Result<RawFd, nix::errno::Errno> {
    let fd: RawFd = nix::fcntl::open("/dev/net/tun", nix::fcntl::OFlag::O_RDWR, nix::sys::stat::Mode::empty())?;
    let mut req = ifreq::from_name(name).unwrap();
    let persist = true;

    unsafe{
        req.set_flags(flags);
        ioctl::tun_set_iff(fd, &req)?;
        ioctl::tun_set_persist(fd, &persist)?;
    }
    Ok(fd)
}

pub fn if_get_flags(name: &str) -> Result<IfFlags, nix::errno::Errno> {
    let req = ifreq::from_name(name).unwrap();
    let flags: IfFlags;

    unsafe{
        ioctl::if_get_flags(get_dummy_socket()?, &req)?;
        flags = req.get_flags();
    }
    Ok(flags)
}

pub fn if_set_flags(name: &str, flags: IfFlags) -> Result<(), nix::errno::Errno> {
    let mut req = ifreq::from_name(name).unwrap();

    unsafe{
        req.set_flags(flags);
        ioctl::if_set_flags(get_dummy_socket()?, &req)?;
    }
    Ok(())
}

pub fn if_set_mtu(name: &str, mtu: libc::c_int) -> Result<(), nix::errno::Errno> {
    let mut req = ifreq::from_name(name).unwrap();

    unsafe{
        req.ifr_ifru.ifr_mtu = mtu;
        ioctl::if_set_mtu(get_dummy_socket()?, &req)?;
    }
    Ok(())
}

pub fn if_set_addr(name: &str, addr: &std::net::Ipv4Addr, netmask: &std::net::Ipv4Addr) -> Result<(), nix::errno::Errno> {
    let mut req = ifreq::from_name(name).unwrap();
    let sock = get_dummy_socket()?;

    let mut sai = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 0,
        sin_addr: libc::in_addr {
            s_addr: u32::from_be_bytes(addr.octets()).to_be()
        },
        sin_zero: [0; 8]
    };
    unsafe {
        req.set_addr(sai);
        ioctl::if_set_addr(sock, &req)?;
    }

    sai.sin_addr.s_addr = u32::from_be_bytes(netmask.octets()).to_be();
    unsafe {
        req.set_addr(sai);
        ioctl::if_set_netmask(sock, &req)?;
    }

    Ok(())
}

fn errno_to_str(err: nix::errno::Errno, msg: String) -> String {
    use nix::errno::Errno;

    let err_str = match err {
        Errno::EPERM => format!("Operation not permitted (EPERM). Try to use sudo"),
        _ => format!("{}", err)
    };

    format!("{}: {}", msg, err_str)
}

pub fn create_tun_interface(name: String, addr: Ipv4Addr, mask: Ipv4Addr, mtu: i32) -> Result<RawFd, String> {


    let iname = name.as_str();
    let tun_fd = tun_create(iname, IfFlags::IFF_TUN | IfFlags::IFF_NO_PI)
        .map_err(|err| errno_to_str(err, format!("Cannot create interface {}", iname)))?;

    let flags = if_get_flags(iname)
        .map_err(|err| errno_to_str(err, format!("Cannot get flags of {}", iname)))?;

    if_set_flags(iname, flags | IfFlags::IFF_UP)
        .map_err(|err| errno_to_str(err, format!("Cannot set flags of {}", iname)))?;

    if_set_mtu(iname, mtu)
        .map_err(|err| errno_to_str(err, format!("Cannot set a MTU of {} to {}", mtu, iname)))?;

    if_set_addr(iname, &addr, &mask)
        .map_err(|err| errno_to_str(err, format!("Cannot set address {} - {} to {}", addr, mask, iname)))?;

    Ok(tun_fd)
}
