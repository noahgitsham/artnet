mod artnet;
mod dmx;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn _ping(addr: IpAddr) {
    let port = artnet::ARTPOLL_UDP_PORT;
    let udp_sock = UdpSocket::bind((addr, port)).expect("Failed to bind");
    // udp_sock.set_broadcast(true).expect("Couldn't set broadcast");

    let node_addr = Ipv4Addr::new(10, 201, 6, 100);

    udp_sock.send_to(&[0, 1, 2], (node_addr, port)).expect("Failed to send data");
}

fn main() -> std::io::Result<()> {
    let _node_addr = Ipv4Addr::new(10, 201, 6, 100);

    let broadcast_addr = artnet::get_likely_broadcast_addr()
        .expect("Couldn't find interface address");

    println!("{}", broadcast_addr);

    for _ in 0..10 {
        artnet::ArtPoll::default(broadcast_addr).poll();
        println!("Polling...");
        sleep(Duration::from_secs(1));
    }

    // println!("{broadcast_addr:?}");
    Ok(())
}
