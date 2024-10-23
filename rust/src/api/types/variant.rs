use flutter_rust_bridge::frb;

use opcua::types::Variant;

use crate::api::types::{
    byte_string::WrapByteString, date_time::WrapDateTime, guid::WrapGuid,
    status_code::WrapStatusCode, string::WrapUAString, string::WrapXmlElement,
};

#[frb(non_opaque)]
pub enum WrapVariant {
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
    DateTime(Box<WrapDateTime>),
    /// Guid
    Guid(Box<WrapGuid>),
    /// StatusCode
    StatusCode(WrapStatusCode),
    /// ByteString
    ByteString(WrapByteString),
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

impl From<Variant> for WrapVariant {
    fn from(value: Variant) -> Self {
        match value {
            Variant::Empty => WrapVariant::Empty,
            Variant::Boolean(v) => WrapVariant::Boolean(v),
            Variant::SByte(v) => WrapVariant::SByte(v),
            Variant::Byte(v) => WrapVariant::Byte(v),
            Variant::Int16(v) => WrapVariant::Int16(v),
            Variant::UInt16(v) => WrapVariant::UInt16(v),
            Variant::Int32(v) => WrapVariant::Int32(v),
            Variant::UInt32(v) => WrapVariant::UInt32(v),
            Variant::Int64(v) => WrapVariant::Int64(v),
            Variant::UInt64(v) => WrapVariant::UInt64(v),
            Variant::Float(v) => WrapVariant::Float(v),
            Variant::Double(v) => WrapVariant::Double(v),
            Variant::String(v) => WrapVariant::String(WrapUAString::from(v)),
            Variant::DateTime(v) => WrapVariant::DateTime(Box::new(WrapDateTime::from(*v))),
            Variant::Guid(v) => WrapVariant::Guid(Box::new(WrapGuid::from(*v))),
            Variant::StatusCode(v) => WrapVariant::StatusCode(WrapStatusCode::from(v)),
            Variant::ByteString(v) => WrapVariant::ByteString(WrapByteString::from(v)),
            Variant::XmlElement(v) => WrapVariant::XmlElement(WrapXmlElement::from(v)),
            _ => unimplemented!(),
        }
    }
}

impl From<WrapVariant> for Variant {
    fn from(value: WrapVariant) -> Self {
        match value {
            WrapVariant::Empty => Variant::Empty,
            WrapVariant::Boolean(v) => Variant::Boolean(v),
            WrapVariant::SByte(v) => Variant::SByte(v),
            WrapVariant::Byte(v) => Variant::Byte(v),
            WrapVariant::Int16(v) => Variant::Int16(v),
            WrapVariant::UInt16(v) => Variant::UInt16(v),
            WrapVariant::Int32(v) => Variant::Int32(v),
            WrapVariant::UInt32(v) => Variant::UInt32(v),
            WrapVariant::Int64(v) => Variant::Int64(v),
            WrapVariant::UInt64(v) => Variant::UInt64(v),
            WrapVariant::Float(v) => Variant::Float(v),
            WrapVariant::Double(v) => Variant::Double(v),
            WrapVariant::String(v) => Variant::String(v.into()),
            WrapVariant::DateTime(v) => Variant::DateTime(Box::new((*v).into())),
            WrapVariant::Guid(v) => Variant::Guid(Box::new((*v).into())),
            WrapVariant::StatusCode(v) => Variant::StatusCode(v.into()),
            WrapVariant::ByteString(v) => Variant::ByteString(v.into()),
            WrapVariant::XmlElement(v) => Variant::XmlElement(v.into()),
            _ => unimplemented!(),
        }
    }
}

#[frb]
pub fn _wrapvariant(_a: WrapVariant) {}
