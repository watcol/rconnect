mod identity;

pub use identity::Identity;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

static PROTOCOL_VERSION: u8 = 7;

/// The packet format
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Packet {
    pub id: u128,
    #[serde(flatten)]
    pub packet_type: PacketType,
}

impl Packet {
    /// Parse received buffer.
    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    /// Convert packet to sendable format.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Create "identity" packet.
    pub fn new_identity() -> Self {
        Self {
            id: gen_id(),
            packet_type: PacketType::Identity(Identity::new()),
        }
    }
}

fn gen_id() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis(),
        Err(_) => panic!("Failed to generate id."),
    }
}

/// Represent three packet types.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "body")]
pub enum PacketType {
    #[serde(rename = "kdeconnect.identity")]
    Identity(Identity),
    #[serde(rename = "kdeconnect.pair")]
    Pair(JsonValue),
    #[serde(rename = "kdeconnect.encrypted")]
    Encrypted(JsonValue),
}
