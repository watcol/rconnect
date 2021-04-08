//! A KDE Connect server/client implementation in Rust
mod packet;
mod device;

pub use packet::Packet;
pub use device::DeviceType;

pub static PORT: u16 = 1716;
