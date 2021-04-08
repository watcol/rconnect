extern crate anyhow;
extern crate rconnect;

use rconnect::{Packet, PORT};
use std::net::UdpSocket;

fn discover() -> anyhow::Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    let local_addr = socket.local_addr()?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(60)))?;
    socket.set_broadcast(true)?;
    let send_packet = Packet::new_identity().to_json()?;
    println!("{}", send_packet);
    socket.send_to(send_packet.as_bytes(), ("255.255.255.255", PORT))?;
    let (size, addr, buf) = loop {
        let mut buf = [0; 10000];
        socket.recv_from(&mut [])?;
        let (size, addr) = socket.recv_from(&mut buf)?;
        if addr != local_addr {
            break (size, addr, buf);
        }
    };
    let received_packet = Packet::from_json(std::str::from_utf8(&buf)?.trim_matches('\u{0}'))?;
    println!("received {} bytes from {}:", size, addr);
    println!("{:#?}", received_packet);
    Ok(())
}

fn main() {
    discover().unwrap();
}
