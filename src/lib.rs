//! A KDE Connect server/client implementation in Rust
mod device;
mod packet;

pub use device::DeviceType;
pub use packet::Packet;

pub static PORT: u16 = 1716;
