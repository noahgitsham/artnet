use std::{ffi::CStr, mem::MaybeUninit, net::Ipv4Addr};

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

    pub fn get_interfaces_iter<'a>() -> NetworkInterfaceIter<'a> {
        let mut ifap = MaybeUninit::<*mut libc::ifaddrs>::uninit();
        match unsafe { libc::getifaddrs(ifap.as_mut_ptr()) } {
            0 => {
                let ifaddrs_init = unsafe { ifap.assume_init() }; // Safe getifaddrs ran
                NetworkInterfaceIter {
                    head: ifaddrs_init,
                    next: unsafe { ifaddrs_init.as_ref() },
                }
            }
            _ => todo!()
        }
    }
}

pub struct NetworkInterfaceIter<'a> {
    head: *mut libc::ifaddrs, // For freeifaddrs on drop
    next: Option<&'a libc::ifaddrs>,
}

pub fn addr_from_sockaddr(sockaddrp: *mut libc::sockaddr) -> Option<Ipv4Addr> {
    let sock = unsafe { sockaddrp.as_ref() };
    sock.map_or(None, |sock| {
        match sock.sa_family as libc::c_int {
            libc::AF_INET => {
                let ip: [i8; 4] = sock.sa_data[2..6]
                    .try_into().unwrap();
                let ip_u = ip.map(|i| i as u8);
                Some(Ipv4Addr::from_octets(ip_u))
            }
            _ => {
                None
            }
        }
    })
}

fn read_next_interface(addr: &libc::ifaddrs) -> Option<&libc::ifaddrs> {
    // Safe as longs as getifaddrs initialised ifaddrs
    unsafe {addr.ifa_next.as_ref() }
}

fn unpack_interface(ifa: &libc::ifaddrs) -> NetworkInterface {
    // Guaranteed to be initialised by getifaddrs
    let name = unsafe { CStr::from_ptr(ifa.ifa_name) }.to_str()
        .unwrap().to_owned();

    NetworkInterface {
        name,
        addr: addr_from_sockaddr(ifa.ifa_addr),
        _subnet_mask: addr_from_sockaddr(ifa.ifa_netmask),
        flags: ifa.ifa_flags,
    }
}

impl<'a> Iterator for NetworkInterfaceIter<'a> {
    type Item = NetworkInterface;
    fn next(&mut self) -> Option<Self::Item> {

        match self.next {
            None => None,
            Some(ifa) => {
                self.next = read_next_interface(ifa);
                Some(unpack_interface(ifa))
            },
        }
    }
}

impl<'a> Drop for NetworkInterfaceIter<'a> {
    fn drop(&mut self) {
        unsafe{ libc::freeifaddrs(self.head) };
    }
}
