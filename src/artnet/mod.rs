mod interface;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};

pub fn get_possible_interfaces() -> impl Iterator<Item=interface::NetworkInterface> {
    use interface::IFF;
    interface::NetworkInterface::get_interfaces_iter()
        .filter(|interface|
            matches!(interface.addr, Some(IpAddr::V4(_)))
            && interface.get_flag(IFF::BROADCAST)
            && interface.get_flag(IFF::UP)
            && interface.get_flag(IFF::MULTICAST)
            && interface.get_flag(IFF::RUNNING)
            && !interface.get_flag(IFF::LOOPBACK)
        )
}

pub fn get_likely_broadcast_addr() -> Result<IpAddr, &'static str> {
    for interface in get_possible_interfaces() {
        if interface.name.starts_with("en") {
            return Ok(interface.addr.expect("This interface should have an address"))
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
pub const ARTNET_VERSION: i8 = 14;
pub const ARTPOLL_UDP_PORT: u16 = 0x1936;
pub const ARTPOLL_ADDR_PRIMARY: SocketAddrV4 = SocketAddrV4::new(
    Ipv4Addr::new(2,255,255,255),
    ARTPOLL_UDP_PORT
);
pub const ARTPOLL_ADDR_SECONDARY: SocketAddrV4 = SocketAddrV4::new(
    Ipv4Addr::new(10,255,255,255),
    ARTPOLL_UDP_PORT
);

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

pub struct ArtPoll {

}

// use crate::dmx::Dmx;
impl ArtPoll {
    pub fn new_poll(addr: IpAddr) {
        let port = ARTPOLL_UDP_PORT;
        let udp_sock = UdpSocket::bind((addr, port)).expect("Failed to bind");
        udp_sock.set_broadcast(true).expect("Couldn't set broadcast");

        let buf = [0; 512];

        udp_sock.send_to(&[0, 1, 2], ARTPOLL_ADDR_SECONDARY)
            .expect("Failed to send data");

    }
}

pub struct ArtPollReply {

}
