use serde::{Serialize, Deserialize};

#[cfg(any(target_os = "ios", target_os = "android"))]
static DEVICE_TYPE: DeviceType = DeviceType::Phone;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
static DEVICE_TYPE: DeviceType = DeviceType::Desktop;

/// Device Type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceType {
    Desktop,
    Laptop,
    #[serde(alias = "smartphone")]
    Phone,
    Tablet,
    Tv,
}

impl Default for DeviceType {
    fn default() -> Self {
        DEVICE_TYPE
    }

}
