use ffi::bindings::*;
use error::*;
use enum_wrappers::unit::*;
use std::ffi::CStr;

/// Fan information readings for an entire S-class unit.
// Checked against local
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FansInfo {
    /// Number of fans in the unit.
    pub count: u32,
    /// Fan data for each fan.
    pub fans: Vec<FanInfo>,
}

impl From<nvmlUnitFanSpeeds_t> for FansInfo {
    fn from(struct_: nvmlUnitFanSpeeds_t) -> Self {
        FansInfo {
            count: struct_.count,
            fans: struct_.fans.iter().map(|f| FanInfo::from(*f)).collect(),
        }
    }
}

/// Fan info reading for a single fan in an S-class unit.
// Checked against local
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FanInfo {
    /// Fan speed (RPM).
    pub speed: u32,
    /// Indicates whether a fan is working properly.
    pub state: FanState,
}

impl From<nvmlUnitFanInfo_t> for FanInfo {
    fn from(struct_: nvmlUnitFanInfo_t) -> Self {
        FanInfo {
            speed: struct_.speed,
            state: struct_.state.into(),
        }
    }
}

/**
Power usage information for an S-class unit. 

The power supply state is a human-readable string that equals "Normal" or contains 
a combination of "Abnormal" plus one or more of the following (aka good luck matching 
on it):

* High voltage
* Fan failure
* Heatsink temperature
* Current limit
* Voltage below UV alarm threshold
* Low-voltage
* SI2C remote off command
* MOD_DISABLE input
* Short pin transition
*/
// Checked against local
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PsuInfo {
    /// PSU current (in A)
    pub current: u32,
    /// PSU power draw (in W)
    pub power_draw: u32,
    /// Human-readable string describing the PSU state.
    pub state: String,
    /// PSU voltage (in V)
    pub voltage: u32,
}

impl PsuInfo {
    /// Waiting for `TryFrom` to be stable. In the meantime, we do this.
    pub fn try_from(struct_: nvmlPSUInfo_t) -> Result<Self> {
        unsafe {
            let state_raw = CStr::from_ptr(struct_.state.as_ptr());
            Ok(PsuInfo {
                current: struct_.current,
                power_draw: struct_.power,
                state: state_raw.to_str()?.into(),
                voltage: struct_.voltage,
            })
        }
    }
}

/// Static S-class unit info.
// Checked against local
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitInfo {
    pub firmware_version: String,
    /// Product identifier.
    pub id: String,
    pub name: String,
    /// Product serial number.
    pub serial: String,
}

impl UnitInfo {
    /// Waiting for `TryFrom` to be stable. In the meantime, we do this.
    pub fn try_from(struct_: nvmlUnitInfo_t) -> Result<Self> {
        unsafe {
            let version_raw = CStr::from_ptr(struct_.firmwareVersion.as_ptr());
            let id_raw = CStr::from_ptr(struct_.id.as_ptr());
            let name_raw = CStr::from_ptr(struct_.name.as_ptr());
            let serial_raw = CStr::from_ptr(struct_.serial.as_ptr());

            Ok(UnitInfo {
                firmware_version: version_raw.to_str()?.into(),
                id: id_raw.to_str()?.into(),
                name: name_raw.to_str()?.into(),
                serial: serial_raw.to_str()?.into(),
            })
        }
    }
}

/// Description of an HWBC entry.
// Checked against local
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HwbcEntry {
    pub id: u32,
    pub firmware_version: String,
}

impl HwbcEntry {
    /// Waiting for `TryFrom` to be stable. In the meantime, we do this.
    pub fn try_from(struct_: nvmlHwbcEntry_t) -> Result<Self> {
        unsafe {
            let version_raw = CStr::from_ptr(struct_.firmwareVersion.as_ptr());
            Ok(HwbcEntry {
                id: struct_.hwbcId,
                firmware_version: version_raw.to_str()?.into()
            })
        }
    }
}
