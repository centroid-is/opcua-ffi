use flutter_rust_bridge::frb;
use opcua::types::{Identifier, NodeId, ObjectId};

use super::{byte_string::WrapByteString, guid::WrapGuid, string::WrapUAString};

#[frb(non_opaque)]
pub enum WrapIdentifier {
    Numeric(u32),
    String(WrapUAString),
    Guid(WrapGuid),
    ByteString(WrapByteString),
}

#[frb(sync)]
impl From<i32> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: i32) -> Self {
        WrapIdentifier::Numeric(v as u32)
    }
}

#[frb(sync)]
impl From<u32> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: u32) -> Self {
        WrapIdentifier::Numeric(v)
    }
}

#[frb(sync)]
impl<'a> From<&'a str> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: &'a str) -> Self {
        WrapIdentifier::from(WrapUAString::from(v))
    }
}

#[frb(sync)]
impl From<&String> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: &String) -> Self {
        WrapIdentifier::from(WrapUAString::from(v))
    }
}

#[frb(sync)]
impl From<String> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: String) -> Self {
        WrapIdentifier::from(WrapUAString::from(v))
    }
}

#[frb(sync)]
impl From<WrapUAString> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: WrapUAString) -> Self {
        WrapIdentifier::String(v)
    }
}

#[frb(sync)]
impl From<WrapGuid> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: WrapGuid) -> Self {
        WrapIdentifier::Guid(v)
    }
}

#[frb(sync)]
impl From<WrapByteString> for WrapIdentifier {
    #[frb(sync)]
    fn from(v: WrapByteString) -> Self {
        WrapIdentifier::ByteString(v)
    }
}

impl Into<Identifier> for WrapIdentifier {
    fn into(self) -> Identifier {
        match self {
            WrapIdentifier::Numeric(n) => Identifier::Numeric(n),
            WrapIdentifier::String(s) => Identifier::String(s.into()),
            WrapIdentifier::Guid(g) => Identifier::Guid(g.into()),
            WrapIdentifier::ByteString(b) => Identifier::ByteString(b.into()),
        }
    }
}

impl From<Identifier> for WrapIdentifier {
    fn from(v: Identifier) -> Self {
        match v {
            Identifier::Numeric(n) => WrapIdentifier::Numeric(n),
            Identifier::String(s) => WrapIdentifier::String(s.into()),
            Identifier::Guid(g) => WrapIdentifier::Guid(g.into()),
            Identifier::ByteString(b) => WrapIdentifier::ByteString(b.into()),
        }
    }
}

#[frb(opaque)]
pub struct WrapNodeId(NodeId);

impl Into<NodeId> for WrapNodeId {
    fn into(self) -> NodeId {
        self.0
    }
}

impl From<NodeId> for WrapNodeId {
    fn from(value: NodeId) -> Self {
        WrapNodeId(value)
    }
}

impl WrapNodeId {
    // Constructs a new NodeId from anything that can be turned into Identifier
    // u32, Guid, ByteString or String
    #[frb(sync)]
    pub fn new(namespace: u16, value: WrapIdentifier) -> Self {
        WrapNodeId(NodeId::new(namespace, value))
    }

    #[frb(sync)]
    /// Returns the node id for the root folder.
    pub fn root_folder_id() -> Self {
        Self(ObjectId::RootFolder.into())
    }

    #[frb(sync)]
    /// Returns the node id for the objects folder.
    pub fn objects_folder_id() -> Self {
        Self(ObjectId::ObjectsFolder.into())
    }

    #[frb(sync)]
    /// Returns the node id for the types folder.
    pub fn types_folder_id() -> Self {
        Self(ObjectId::TypesFolder.into())
    }

    #[frb(sync)]
    /// Returns the node id for the views folder.
    pub fn views_folder_id() -> Self {
        Self(ObjectId::ViewsFolder.into())
    }

    #[frb(sync)]
    /// Test if the node id is null, i.e. 0 namespace and 0 identifier
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[frb(sync)]
    /// Returns a null node id
    pub fn null() -> Self {
        Self(NodeId::null())
    }

    #[frb(sync)]
    // Creates a numeric node id with an id incrementing up from 1000
    pub fn next_numeric(namespace: u16) -> Self {
        Self(NodeId::next_numeric(namespace))
    }

    // /// Extracts an ObjectId from a node id, providing the node id holds an object id
    // pub fn as_object_id(&self) -> std::result::Result<ObjectId, NodeIdError> {

    // }

    // pub fn as_reference_type_id(&self) -> std::result::Result<ReferenceTypeId, NodeIdError> {
    // }

    #[frb(sync)]
    /// Test if the node id is numeric
    pub fn is_numeric(&self) -> bool {
        self.0.is_numeric()
    }

    #[frb(sync)]
    /// Test if the node id is a string
    pub fn is_string(&self) -> bool {
        self.0.is_string()
    }

    #[frb(sync)]
    /// Test if the node id is a guid
    pub fn is_guid(&self) -> bool {
        self.0.is_guid()
    }

    #[frb(sync)]
    /// Test if the node id us a byte string
    pub fn is_byte_string(&self) -> bool {
        self.0.is_byte_string()
    }
}

#[frb]
pub fn _wrapidentifier(_a: WrapIdentifier) {}

#[frb]
pub fn _wrapnodeid(_a: WrapNodeId) {}
