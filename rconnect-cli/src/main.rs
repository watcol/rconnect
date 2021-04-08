extern crate anyhow;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Packet {
    id: u128,
    #[serde(flatten)]
    packet_type: PacketType,
}

impl Packet {
    fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "body")]
enum PacketType {
    #[serde(rename = "kdeconnect.identity")]
    Identity(Identity),
    #[serde(rename = "kdeconnect.pair")]
    Pair(JsonValue),
    #[serde(rename = "kdeconnect.encrypted")]
    Encrypted(JsonValue),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identity {
    device_name: String,
    device_type: DeviceType,
    protocol_version: u8,
    tcp_port: u16,
    incoming_capabilities: Vec<String>,
    outgoing_capabilities: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum DeviceType {
    Desktop,
    Laptop,
    #[serde(alias = "smartphone")]
    Phone,
    Tablet,
    Tv,
}

use std::net::UdpSocket;

fn discover() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:1716")?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(60)))?;
    socket.set_broadcast(true)?;
    let mut buf = [0; 10000];
    let (size, addr) = socket.recv_from(&mut buf)?;
    println!("received {} bytes from {}:", size, addr);
    let buf = std::str::from_utf8(&buf)?.trim_matches('\u{0}');
    let packet = Packet::from_json(buf)?;
    println!("{:#?}", packet);

    Ok(())
}

fn main() {
    discover().unwrap();
}
