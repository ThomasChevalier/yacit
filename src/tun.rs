use nix;
use ifstructs::ifreq;
use libc;
use std::os::unix::prelude::RawFd;

// From https://github.com/torvalds/linux/blob/master/include/uapi/linux/if_tun.h#L66
pub const IFF_TUN: libc::c_short = 0x0001;
pub const IFF_NO_PI: libc::c_short = 0x1000;

// From https://github.com/torvalds/linux/blob/master/include/uapi/linux/if_tun.h#L34
const TUN_IOC_MAGIC: u8 = 'T' as u8;
const TUN_IOC_SET_IFF: u8 = 202;
const TUN_IOC_SET_PERSIST: u8 = 203;

unsafe fn tun_set_iff(fd: i32, data: *const ifreq) -> Result<i32, nix::errno::Errno> {
    let res = libc::ioctl(fd, nix::request_code_write!(TUN_IOC_MAGIC, TUN_IOC_SET_IFF, std::mem::size_of::<i32>()), data);
    nix::errno::Errno::result(res)
}

unsafe fn tun_set_persist(fd: i32, data: *const bool) -> Result<i32, nix::errno::Errno> {
    let res = libc::ioctl(fd, nix::request_code_write!(TUN_IOC_MAGIC, TUN_IOC_SET_PERSIST, std::mem::size_of::<i32>()), data);
    nix::errno::Errno::result(res)
}


pub fn tun_create(name: &str, flags: libc::c_short) -> Result<(), nix::errno::Errno> {
    let fd: RawFd = nix::fcntl::open("/dev/net/tun", nix::fcntl::OFlag::O_RDWR, nix::sys::stat::Mode::empty())?;

    let mut req = ifreq::from_name(name).unwrap();
    req.set_flags(flags);
    let persist = true;

    unsafe{
        tun_set_iff(fd, &req)?;
        tun_set_persist(fd, &persist)?;
    }
    Ok(())
}
