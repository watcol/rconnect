extern crate anyhow;
extern crate dns_lookup;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

static PORT: u16 = 1716;
#[cfg(any(target_os = "ios", target_os = "android"))]
static DEVICE_TYPE: DeviceType = DeviceType::Phone;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
static DEVICE_TYPE: DeviceType = DeviceType::Desktop;

static PROTOCOL_VERSION: u8 = 7;

fn gen_id() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis(),
        Err(_) => panic!("Failed to generate id."),
    }
}

fn device_name() -> String {
    match dns_lookup::get_hostname() {
        Ok(hostname) => hostname,
        Err(_) => String::from("fallback"),
    }
}

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

    fn new_identity() -> Self {
        Self {
            id: gen_id(),
            packet_type: PacketType::new_identity(),
        }
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

impl PacketType {
    fn new_identity() -> Self {
        Self::Identity(Identity::new())
    }
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

impl Identity {
    fn new() -> Self {
        Self {
            device_name: device_name(),
            device_type: DEVICE_TYPE,
            protocol_version: PROTOCOL_VERSION,
            tcp_port: PORT,
            incoming_capabilities: Vec::new(),
            outgoing_capabilities: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    let local_addr = socket.local_addr()?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(60)))?;
    socket.set_broadcast(true)?;
    let send_packet = Packet::new_identity().to_json()?;
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
