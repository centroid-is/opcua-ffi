use flutter_rust_bridge::frb;

use crate::api::types::{date_time::UADateTime, status_code::StatusCode, variant::Variant};

#[frb(non_opaque)]
pub struct DataValue {
    pub value: Option<Variant>,
    /// The status associated with the value.
    /// Not present if the StatusCode bit in the EncodingMask is False
    pub status: Option<StatusCode>,
    /// The source timestamp associated with the value.
    /// Not present if the SourceTimestamp bit in the EncodingMask is False.
    pub source_timestamp: Option<UADateTime>,
    /// The number of 10 picosecond intervals for the SourceTimestamp.
    /// Not present if the SourcePicoSeconds bit in the EncodingMask is False.
    /// If the source timestamp is missing the picoseconds are ignored.
    pub source_picoseconds: Option<u16>,
    /// The Server timestamp associated with the value.
    /// Not present if the ServerTimestamp bit in the EncodingMask is False.
    pub server_timestamp: Option<UADateTime>,
    /// The number of 10 picosecond intervals for the ServerTimestamp.
    /// Not present if the ServerPicoSeconds bit in the EncodingMask is False.
    /// If the Server timestamp is missing the picoseconds are ignored.
    pub server_picoseconds: Option<u16>,
}

impl From<opcua::types::DataValue> for DataValue {
    fn from(value: opcua::types::DataValue) -> Self {
        DataValue {
            value: value.value.map(Variant::from),
            status: value.status.map(StatusCode::from),
            source_timestamp: value.source_timestamp.map(UADateTime::from),
            source_picoseconds: value.source_picoseconds,
            server_timestamp: value.server_timestamp.map(UADateTime::from),
            server_picoseconds: value.server_picoseconds,
        }
    }
}

impl From<DataValue> for opcua::types::DataValue {
    fn from(value: DataValue) -> Self {
        opcua::types::DataValue {
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
pub fn _wrapdatavalue(_a: DataValue) {}
