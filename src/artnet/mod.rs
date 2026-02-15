mod interface;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};

pub fn get_possible_interfaces() -> impl Iterator<Item=interface::NetworkInterface> {
    use interface::IFF;
    interface::NetworkInterface::get_interfaces_iter()
        .filter(|interface|
            interface.addr.is_some()
            && interface.get_flag(IFF::BROADCAST)
            && interface.get_flag(IFF::UP)
            && interface.get_flag(IFF::MULTICAST)
            && interface.get_flag(IFF::RUNNING)
            && !interface.get_flag(IFF::LOOPBACK)
        )
}

pub fn get_likely_broadcast_addr() -> Result<Ipv4Addr, &'static str> {
    for interface in get_possible_interfaces() {
        if interface.name.starts_with("en") {
            return Ok(interface.addr.expect("This interface should have an address")
                .try_into().unwrap())
        }
    }
    Err("No address found")
}

const ARTNET_ID: [u8; 8] = [
    b'A',
    b'r',
    b't',
    b'-',
    b'N',
    b'e',
    b't',
    0,
];
pub const ARTNET_VERSION: u16 = 14;

#[allow(unused)]
pub enum OpCodes {
    OpPoll      = 0x2000,
    OpPollReply = 0x2100,

    OpDiagData = 0x2300,
    OpCommand = 0x2400,

    OpDataRequest = 0x2700,
    OpDataReply   = 0x2800,

    OpDmx = 0x5000,
    OpNzs = 0x5100,
    OpSync = 0x5200,
    OpAddress = 0x6000,
    OpInput = 0x7000,

    OpTodRequest = 0x8000,
    OpTodData    = 0x8100,
    OpTodControl = 0x8200,

    OpRdm    = 0x8300,
    OpRdmSub = 0x8400,

    OpVideoSetup   = 0xa010,
    OpVideoPalette = 0xa020,
    OpVideoData    = 0xa040,

    OpMacMaster = 0xf000,
    OpMacSlave  = 0xf100,

    OpFirmwareMaster = 0xf200,
    OpFirmwareReply  = 0xf300,

    OpFileTnMaster = 0xf400,
    OpFileFnMaster = 0xf500,
    OpFileFnReply  = 0xf600,

    OpIpProg      = 0xf800,
    OpIpProgReply = 0xf900,

    OpMedia            = 0x9000,
    OpMediaPatch       = 0x9100,
    OpMediaControl     = 0x9200,
    OpMediaContrlReply = 0x9300,

    OpTimeCode = 0x9700,
    OpTimeSync = 0x9800,

    OpTrigger = 0x9900,

    OpDirectory      = 0x9a00,
    OpDirectoryReply = 0x9b00,
}

#[allow(unused)]
pub enum PriorityCodes {
    DpLow = 0x10,
    DpMed = 0x40,
    DpHigh = 0x80,
    DpCritical = 0xe0,
    DpVolatile = 0xf0,
}

pub const ARTPOLL_UDP_PORT: u16 = 0x1936;
pub const ARTPOLL_ADDR_PRIMARY: SocketAddrV4 = SocketAddrV4::new(
    Ipv4Addr::new(2,255,255,255),
    ARTPOLL_UDP_PORT
);
pub const ARTPOLL_ADDR_SECONDARY: SocketAddrV4 = SocketAddrV4::new(
    Ipv4Addr::new(10,255,255,255),
    ARTPOLL_UDP_PORT
);

pub struct ArtPoll {
    broadcast_addr: Ipv4Addr,
    poll_addr: SocketAddrV4,
}

// use crate::dmx::Dmx;
impl ArtPoll {
    pub fn _primary(broadcast_addr: Ipv4Addr) -> Self {
        Self {
            broadcast_addr,
            poll_addr: ARTPOLL_ADDR_PRIMARY
        }
    }
    pub fn secondary(broadcast_addr: Ipv4Addr) -> Self {
        Self {
            broadcast_addr,
            poll_addr: ARTPOLL_ADDR_SECONDARY
        }
    }
    pub fn default(broadcast_addr: Ipv4Addr) -> Self {
        ArtPoll::secondary(broadcast_addr)
    }

    pub fn poll(&self) {
        let udp_sock = UdpSocket::bind((self.broadcast_addr, ARTPOLL_UDP_PORT)).expect("Failed to bind to ArtPoll address");
        udp_sock.set_broadcast(true).expect("Couldn't set broadcast");
        udp_sock.set_multicast_loop_v4(false).expect("Couldn't set broadcast");
        // println!("{:?}", udp_sock.multicast_loop_v4());

        const ARTPOLL_LENGTH: usize =
            8 + // ID
            2 + // Opcode
            1 + // ProtVersion
            1 +
            1 + // Flags
            1 + // Diag
            1   // TargetPort
            // +
            // 1 +
            // 1 +
            // 1 +
            // 1 + // Esta
            // 1 +
            // 1 + // Oem
            // 1
            ;

        let mut buf: [u8; ARTPOLL_LENGTH] = [0; ARTPOLL_LENGTH];

        let flags: u8 = 0b00000100;

        buf[0..8].copy_from_slice(&ARTNET_ID[0..8]);
        buf[8..10].copy_from_slice(&(OpCodes::OpPoll as u16).to_le_bytes());
        buf[10..12].copy_from_slice(&ARTNET_VERSION.to_be_bytes());
        buf[12..13].copy_from_slice(&flags.to_be_bytes());
        buf[13..14].copy_from_slice(&(PriorityCodes::DpLow as u8).to_le_bytes());
        // buf[14..18].copy_from_slice(&[0,0,0,0]);
        // let esta_manufacturer: u16 = 6969;
        // buf[18..20].copy_from_slice(&esta_manufacturer.to_be_bytes());
        // let oem_code: u16 = 6969;
        // buf[20..22].copy_from_slice(&oem_code.to_be_bytes());

        // buf.reverse();

        udp_sock.send_to(&buf, self.poll_addr)
            .expect("Failed to send ArtPoll");

    }
}

// pub struct ArtPollReply {
//
// }
