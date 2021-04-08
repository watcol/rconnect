use serde::{Serialize, Deserialize};
use crate::DeviceType;

/// "identity" packet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub protocol_version: u8,
    pub tcp_port: u16,
    pub incoming_capabilities: Vec<String>,
    pub outgoing_capabilities: Vec<String>,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            // Temporary device id
            device_id: String::from("DEVICE_ID"),
            device_name: device_name(),
            device_type: DeviceType::default(),
            protocol_version: super::PROTOCOL_VERSION,
            tcp_port: crate::PORT,
            incoming_capabilities: Vec::new(),
            outgoing_capabilities: Vec::new(),
        }
    }
}

impl Identity {
    // Create "identity" packet.
    pub fn new() -> Self {
        Self::default()
    }
}

fn device_name() -> String {
    match dns_lookup::get_hostname() {
        Ok(hostname) => hostname,
        Err(_) => String::from("fallback"),
    }
}

