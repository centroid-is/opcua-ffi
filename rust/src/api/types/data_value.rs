use flutter_rust_bridge::frb;

use opcua::types::{DataValue, StatusCode};

use crate::api::types::{date_time::WrapDateTime, variant::WrapVariant};

#[frb(non_opaque)]
pub struct WrapDataValue {
    pub value: Option<WrapVariant>,
    /// The status associated with the value.
    /// Not present if the StatusCode bit in the EncodingMask is False
    pub status: Option<StatusCode>,
    /// The source timestamp associated with the value.
    /// Not present if the SourceTimestamp bit in the EncodingMask is False.
    pub source_timestamp: Option<WrapDateTime>,
    /// The number of 10 picosecond intervals for the SourceTimestamp.
    /// Not present if the SourcePicoSeconds bit in the EncodingMask is False.
    /// If the source timestamp is missing the picoseconds are ignored.
    pub source_picoseconds: Option<u16>,
    /// The Server timestamp associated with the value.
    /// Not present if the ServerTimestamp bit in the EncodingMask is False.
    pub server_timestamp: Option<WrapDateTime>,
    /// The number of 10 picosecond intervals for the ServerTimestamp.
    /// Not present if the ServerPicoSeconds bit in the EncodingMask is False.
    /// If the Server timestamp is missing the picoseconds are ignored.
    pub server_picoseconds: Option<u16>,
}

impl From<DataValue> for WrapDataValue {
    fn from(value: DataValue) -> Self {
        WrapDataValue {
            value: value.value.map(WrapVariant::from),
            status: value.status.map(StatusCode::from),
            source_timestamp: value.source_timestamp.map(WrapDateTime::from),
            source_picoseconds: value.source_picoseconds,
            server_timestamp: value.server_timestamp.map(WrapDateTime::from),
            server_picoseconds: value.server_picoseconds,
        }
    }
}

impl From<WrapDataValue> for DataValue {
    fn from(value: WrapDataValue) -> Self {
        DataValue {
            value: value.value.map(|v| v.into()),
            status: value.status.map(|v| v.into()),
            source_timestamp: value.source_timestamp.map(|v| v.into()),
            source_picoseconds: value.source_picoseconds,
            server_timestamp: value.server_timestamp.map(|v| v.into()),
            server_picoseconds: value.server_picoseconds,
        }
    }
}

#[frb]
pub fn _wrapdatavalue(_a: WrapDataValue) {}
