use std::{ffi::CStr, mem::MaybeUninit, net::{Ipv4Addr}};

#[allow(nonstandard_style, unused)]
pub enum IFF {
    UP          = libc::IFF_UP as isize,
    BROADCAST   = libc::IFF_BROADCAST as isize,
    DEBUG       = libc::IFF_DEBUG as isize,
    LOOPBACK    = libc::IFF_LOOPBACK as isize,
    POINTOPOINT = libc::IFF_POINTOPOINT as isize,
    NOTRAILERS  = libc::IFF_NOTRAILERS as isize,
    RUNNING     = libc::IFF_RUNNING as isize,
    NOARP       = libc::IFF_NOARP as isize,
    PROMISC     = libc::IFF_PROMISC as isize,
    ALLMULTI    = libc::IFF_ALLMULTI as isize,
    MASTER      = libc::IFF_MASTER as isize,
    SLAVE       = libc::IFF_SLAVE as isize,
    MULTICAST   = libc::IFF_MULTICAST as isize,
    PORTSEL     = libc::IFF_PORTSEL as isize,
    AUTOMEDIA   = libc::IFF_AUTOMEDIA as isize,
    DYNAMIC     = libc::IFF_DYNAMIC as isize,
    LOWER_UP    = libc::IFF_LOWER_UP as isize,
    DORMANT     = libc::IFF_DORMANT as isize,
    ECHO        = libc::IFF_ECHO as isize,
}

#[derive(Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub addr: Option<Ipv4Addr>,
    pub _subnet_mask: Option<Ipv4Addr>,
    pub flags: u32,
}

impl NetworkInterface {
    pub fn get_flag(&self, flag: IFF) -> bool {
        (self.flags & (flag as u32)) != 0
    }

    pub fn get_interfaces_iter() -> NetworkInterfaceIter {
        let mut ifaddrs = MaybeUninit::<*mut libc::ifaddrs>::uninit();
        match unsafe { libc::getifaddrs(ifaddrs.as_mut_ptr()) } {
            0 => {
                NetworkInterfaceIter {
                    head: unsafe { ifaddrs.assume_init() },
                    next: unsafe { ifaddrs.assume_init_read() },
                }
            }
            _ => todo!()
        }
    }
}

pub struct NetworkInterfaceIter {
    head: *mut libc::ifaddrs,
    next: *mut libc::ifaddrs,
}

pub fn addr_from_c_sock(sock: *mut libc::sockaddr) -> Option<Ipv4Addr> {
    if sock.is_null() { return None }
    let addr = unsafe {*sock};
    match addr.sa_family as libc::c_int {
        libc::AF_INET => {
            // println!("{:?}, ", addr.sa_data as [u8; 14]);
            let ip: [i8; 4] = addr.sa_data[2..6]
                .try_into().unwrap();
            let ip_u = ip.map(|i| i as u8);
            Some(Ipv4Addr::from_octets(ip_u))
        }
        // libc::AF_INET6 => {
        //     println!("{:?}, ", addr.sa_data);
        //     Some(IpAddr::V4(Ipv4Addr::new(0,0,0,0)))
        // }
        _ => {
            None
        }
    }
}

impl Iterator for NetworkInterfaceIter {
    type Item = NetworkInterface;
    fn next(&mut self) -> Option<Self::Item> {

        if self.next.is_null() {
            None
        } else {
            let ifa = unsafe {*self.next};
            self.next = ifa.ifa_next;

            let name = unsafe { CStr::from_ptr(ifa.ifa_name) }.to_str()
                .unwrap().to_owned();

            Some(NetworkInterface {
                name,
                addr: addr_from_c_sock(ifa.ifa_addr),
                _subnet_mask: addr_from_c_sock(ifa.ifa_netmask),
                flags: ifa.ifa_flags,
            })
        }
    }
}

impl Drop for NetworkInterfaceIter {
    fn drop(&mut self) {
        unsafe{ libc::freeifaddrs(self.head) };
    }
}
