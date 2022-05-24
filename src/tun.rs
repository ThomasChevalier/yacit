use nix;
use ifstructs::ifreq;
use libc;
use std::os::unix::prelude::RawFd;

// FLAGS

// linux/include/uapi/linux/if_tun.h
pub const IFF_TUN: libc::c_short = libc::IFF_TUN as  libc::c_short;
pub const IFF_NO_PI: libc::c_short = libc::IFF_NO_PI as  libc::c_short;

// linux/include/uapi/linux/if.h
pub const IFF_UP: libc::c_short = libc::IFF_UP as libc::c_short;

mod ioctl{
    use ifstructs::ifreq;
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
}

fn get_dummy_socket() -> Result<RawFd, nix::errno::Errno> {
    use nix::sys::socket::*;
    socket(
        AddressFamily::Inet, SockType::Datagram, 
        SockFlag::empty(), None)
}

pub fn tun_create(name: &str, flags: libc::c_short) -> Result<RawFd, nix::errno::Errno> {
    let fd: RawFd = nix::fcntl::open("/dev/net/tun", nix::fcntl::OFlag::O_RDWR, nix::sys::stat::Mode::empty())?;

    let mut req = ifreq::from_name(name).unwrap();
    req.set_flags(flags);
    let persist = true;

    unsafe{
        ioctl::tun_set_iff(fd, &req)?;
        ioctl::tun_set_persist(fd, &persist)?;
    }
    Ok(fd)
}

pub fn if_get_flags(name: &str) -> Result<libc::c_short, nix::errno::Errno> {
    let req = ifreq::from_name(name).unwrap();

    unsafe{
        ioctl::if_get_flags(get_dummy_socket()?, &req)?;
    }
    Ok(req.get_flags())
}

pub fn if_set_flags(name: &str, flags: libc::c_short) -> Result<(), nix::errno::Errno> {
    let mut req = ifreq::from_name(name).unwrap();
    req.set_flags(flags);

    unsafe{
        ioctl::if_set_flags(get_dummy_socket()?, &req)?;
    }
    Ok(())
}
