extern crate anyhow;

use std::net::UdpSocket;

fn discover() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:1716")?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(60)))?;
    socket.set_broadcast(true)?;
    let mut buf = [0; 10000];
    let (size, addr) = socket.recv_from(&mut buf)?;
    println!("received {} bytes from {}:", size, addr);
    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}

fn main() {
    discover().unwrap();
}
