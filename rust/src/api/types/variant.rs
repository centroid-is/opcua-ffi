use flutter_rust_bridge::frb;

use crate::api::types::{
    byte_string::ByteString, date_time::UADateTime, guid::WrapGuid, status_code::StatusCode,
    string::WrapUAString, string::WrapXmlElement,
};

#[frb(non_opaque)]
pub enum Variant {
    /// Empty type has no value. It is equivalent to a Null value (part 6 5.1.6)
    // #[default]
    Empty,
    /// Boolean
    Boolean(bool),
    /// Signed byte
    SByte(i8),
    /// Unsigned byte
    Byte(u8),
    /// Signed 16-bit int
    Int16(i16),
    /// Unsigned 16-bit int
    UInt16(u16),
    /// Signed 32-bit int
    Int32(i32),
    /// Unsigned 32-bit int
    UInt32(u32),
    /// Signed 64-bit int
    Int64(i64),
    /// Unsigned 64-bit int
    UInt64(u64),
    /// Float
    Float(f32),
    /// Double
    Double(f64),
    /// String
    String(WrapUAString),
    /// DateTime
    DateTime(Box<UADateTime>),
    /// Guid
    Guid(Box<WrapGuid>),
    /// StatusCode
    StatusCode(StatusCode),
    /// ByteString
    ByteString(ByteString),
    /// XmlElement
    XmlElement(WrapXmlElement),
    // /// QualifiedName
    // QualifiedName(Box<QualifiedName>),
    // /// LocalizedText
    // LocalizedText(Box<LocalizedText>),
    // /// NodeId
    // NodeId(Box<NodeId>),
    // /// ExpandedNodeId
    // ExpandedNodeId(Box<ExpandedNodeId>),
    // /// ExtensionObject
    // ExtensionObject(Box<ExtensionObject>),
    // // Variant
    // Variant(Box<Variant>),
    // // DataValue
    // DataValue(Box<DataValue>),
    // // Diagnostics
    // DiagnosticInfo(Box<DiagnosticInfo>),
    // /// Single dimension array which can contain any scalar type, all the same type. Nested
    // /// arrays will be rejected.
    // Array(Box<Array>),
}

impl From<opcua::types::Variant> for Variant {
    fn from(value: opcua::types::Variant) -> Self {
        match value {
            opcua::types::Variant::Empty => Variant::Empty,
            opcua::types::Variant::Boolean(v) => Variant::Boolean(v),
            opcua::types::Variant::SByte(v) => Variant::SByte(v),
            opcua::types::Variant::Byte(v) => Variant::Byte(v),
            opcua::types::Variant::Int16(v) => Variant::Int16(v),
            opcua::types::Variant::UInt16(v) => Variant::UInt16(v),
            opcua::types::Variant::Int32(v) => Variant::Int32(v),
            opcua::types::Variant::UInt32(v) => Variant::UInt32(v),
            opcua::types::Variant::Int64(v) => Variant::Int64(v),
            opcua::types::Variant::UInt64(v) => Variant::UInt64(v),
            opcua::types::Variant::Float(v) => Variant::Float(v),
            opcua::types::Variant::Double(v) => Variant::Double(v),
            opcua::types::Variant::String(v) => Variant::String(WrapUAString::from(v)),
            opcua::types::Variant::DateTime(v) => Variant::DateTime(Box::new(UADateTime::from(*v))),
            opcua::types::Variant::Guid(v) => Variant::Guid(Box::new(WrapGuid::from(*v))),
            opcua::types::Variant::StatusCode(v) => Variant::StatusCode(StatusCode::from(v)),
            opcua::types::Variant::ByteString(v) => Variant::ByteString(ByteString::from(v)),
            opcua::types::Variant::XmlElement(v) => Variant::XmlElement(WrapXmlElement::from(v)),
            _ => unimplemented!(),
        }
    }
}

impl From<Variant> for opcua::types::Variant {
    fn from(value: Variant) -> Self {
        match value {
            Variant::Empty => opcua::types::Variant::Empty,
            Variant::Boolean(v) => opcua::types::Variant::Boolean(v),
            Variant::SByte(v) => opcua::types::Variant::SByte(v),
            Variant::Byte(v) => opcua::types::Variant::Byte(v),
            Variant::Int16(v) => opcua::types::Variant::Int16(v),
            Variant::UInt16(v) => opcua::types::Variant::UInt16(v),
            Variant::Int32(v) => opcua::types::Variant::Int32(v),
            Variant::UInt32(v) => opcua::types::Variant::UInt32(v),
            Variant::Int64(v) => opcua::types::Variant::Int64(v),
            Variant::UInt64(v) => opcua::types::Variant::UInt64(v),
            Variant::Float(v) => opcua::types::Variant::Float(v),
            Variant::Double(v) => opcua::types::Variant::Double(v),
            Variant::String(v) => opcua::types::Variant::String(v.into()),
            Variant::DateTime(v) => opcua::types::Variant::DateTime(Box::new((*v).into())),
            Variant::Guid(v) => opcua::types::Variant::Guid(Box::new((*v).into())),
            Variant::StatusCode(v) => opcua::types::Variant::StatusCode(v.into()),
            Variant::ByteString(v) => opcua::types::Variant::ByteString(v.into()),
            Variant::XmlElement(v) => opcua::types::Variant::XmlElement(v.into()),
            // _ => unimplemented!(),
        }
    }
}

#[frb]
pub fn _wrapvariant(_a: Variant) {}
