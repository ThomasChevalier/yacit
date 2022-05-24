use libc;
use std::io;

pub type IfName = [u8; libc::IFNAMSIZ];

impl ifreq {
    pub fn from_name(name: &str) -> io::Result<ifreq> {
        let mut req: ifreq = unsafe { std::mem::zeroed() };
        req.set_name(name)?;
        Ok(req)
    }
}

pub type IfFlags = nix::net::if_::InterfaceFlags;

#[repr(C)]
pub union ifr_ifru {
    pub ifr_addr: libc::sockaddr_in,
    pub ifr_dstaddr: libc::sockaddr,
    pub ifr_broadaddr: libc::sockaddr,
    pub ifr_netmask: libc::sockaddr,
    pub ifr_hwaddr: libc::sockaddr,
    pub ifr_flags: libc::c_short,
    pub ifr_ifindex: libc::c_int,
    pub ifr_metric: libc::c_int,
    pub ifr_mtu: libc::c_int,
    pub ifr_map: ifmap,
    pub ifr_slave: IfName,
    pub ifr_newname: IfName,
    pub ifr_data: *mut libc::c_char,
}

#[repr(C)]
pub struct ifreq {
    pub ifr_name: IfName,
    pub ifr_ifru: ifr_ifru,
}

macro_rules! set_name {
    ($name_field:expr, $name_str:expr) => {{
        let name_c = &::std::ffi::CString::new($name_str.to_owned()).map_err(|_| {
            ::std::io::Error::new(
                ::std::io::ErrorKind::InvalidInput,
                "malformed interface name",
            )
        })?;
        let name_slice = name_c.as_bytes_with_nul();
        if name_slice.len() > libc::IFNAMSIZ {
            return Err(io::Error::new(::std::io::ErrorKind::InvalidInput, "").into());
        }
        $name_field[..name_slice.len()].clone_from_slice(name_slice);

        Ok(())
    }};
}

macro_rules! get_name {
    ($name_field:expr) => {{
        let nul_pos = match $name_field.iter().position(|x| *x == 0) {
            Some(p) => p,
            None => {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    "malformed interface name",
                )
                .into())
            }
        };

        ::std::ffi::CString::new(&$name_field[..nul_pos])
            .unwrap()
            .into_string()
            .map_err(|_| {
                ::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    "malformed interface name",
                )
            })
    }};
}

impl ifreq {
    pub fn set_name(&mut self, name: &str) -> io::Result<()> {
        set_name!(self.ifr_name, name)
    }

    pub fn get_name(&self) -> io::Result<String> {
        get_name!(self.ifr_name)
    }

    /// Get flags
    pub unsafe fn get_flags(&self) -> IfFlags {
        IfFlags::from_bits_truncate(i32::from(self.ifr_ifru.ifr_flags))
    }

    /// Enable passed flags
    pub unsafe fn set_flags(&mut self, flags: IfFlags) {
        self.ifr_ifru.ifr_flags = flags.bits() as i16;
    }

    /// Enable passed flags
    pub unsafe fn set_raw_flags(&mut self, raw_flags: libc::c_short) {
        self.ifr_ifru.ifr_flags = raw_flags;
    }

    pub unsafe fn set_addr(&mut self, addr: libc::sockaddr_in) {
        self.ifr_ifru.ifr_addr = addr;
    }

    pub unsafe fn set_iface_index(&mut self, idx: libc::c_int) {
        self.ifr_ifru.ifr_ifindex = idx;
    }

    pub unsafe fn get_iface_index(&mut self) -> libc::c_int {
        self.ifr_ifru.ifr_ifindex
    }
}


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}