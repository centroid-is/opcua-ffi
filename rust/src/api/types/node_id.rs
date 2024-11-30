use flutter_rust_bridge::frb;
use opcua::types::ObjectId;

use super::{
    byte_string::ByteString, guid::UAGuid,
    monitored_item_create_request::MonitoredItemCreateRequest, string::UAString,
};

#[frb(non_opaque)]
pub enum Identifier {
    Numeric(u32),
    String(UAString),
    Guid(UAGuid),
    ByteString(ByteString),
}

#[frb(sync)]
impl From<i32> for Identifier {
    #[frb(sync, positional)]
    fn from(v: i32) -> Self {
        Identifier::Numeric(v as u32)
    }
}

#[frb(sync)]
impl From<u32> for Identifier {
    #[frb(sync, positional)]
    fn from(v: u32) -> Self {
        Identifier::Numeric(v)
    }
}

#[frb(sync)]
impl<'a> From<&'a str> for Identifier {
    #[frb(sync, positional)]
    fn from(v: &'a str) -> Self {
        Identifier::from(UAString::from(v))
    }
}

#[frb(sync)]
impl From<&String> for Identifier {
    #[frb(sync, positional)]
    fn from(v: &String) -> Self {
        Identifier::from(UAString::from(v))
    }
}

#[frb(sync)]
impl From<String> for Identifier {
    #[frb(sync, positional)]
    fn from(v: String) -> Self {
        Identifier::from(UAString::from(v))
    }
}

#[frb(sync)]
impl From<UAString> for Identifier {
    #[frb(sync, positional)]
    fn from(v: UAString) -> Self {
        Identifier::String(v)
    }
}

#[frb(sync)]
impl From<UAGuid> for Identifier {
    #[frb(sync, positional)]
    fn from(v: UAGuid) -> Self {
        Identifier::Guid(v)
    }
}

#[frb(sync)]
impl From<ByteString> for Identifier {
    #[frb(sync, positional)]
    fn from(v: ByteString) -> Self {
        Identifier::ByteString(v)
    }
}

impl Into<opcua::types::Identifier> for Identifier {
    fn into(self) -> opcua::types::Identifier {
        match self {
            Identifier::Numeric(n) => opcua::types::Identifier::Numeric(n),
            Identifier::String(s) => opcua::types::Identifier::String(s.into()),
            Identifier::Guid(g) => opcua::types::Identifier::Guid(g.into()),
            Identifier::ByteString(b) => opcua::types::Identifier::ByteString(b.into()),
        }
    }
}

impl From<opcua::types::Identifier> for Identifier {
    fn from(v: opcua::types::Identifier) -> Self {
        match v {
            opcua::types::Identifier::Numeric(n) => Identifier::Numeric(n),
            opcua::types::Identifier::String(s) => Identifier::String(s.into()),
            opcua::types::Identifier::Guid(g) => Identifier::Guid(g.into()),
            opcua::types::Identifier::ByteString(b) => Identifier::ByteString(b.into()),
        }
    }
}

#[frb(opaque)]
pub struct NodeId(opcua::types::NodeId);

impl Into<opcua::types::NodeId> for NodeId {
    fn into(self) -> opcua::types::NodeId {
        self.0
    }
}

impl From<opcua::types::NodeId> for NodeId {
    fn from(value: opcua::types::NodeId) -> Self {
        NodeId(value)
    }
}

impl NodeId {
    // Constructs a new NodeId from anything that can be turned into Identifier
    // u32, Guid, ByteString or String
    #[frb(sync)]
    pub fn new(namespace: u16, value: Identifier) -> Self {
        NodeId(opcua::types::NodeId::new(namespace, value))
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
        Self(opcua::types::NodeId::null())
    }

    #[frb(sync)]
    // Creates a numeric node id with an id incrementing up from 1000
    pub fn next_numeric(namespace: u16) -> Self {
        Self(opcua::types::NodeId::next_numeric(namespace))
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

    #[frb(sync)]
    pub fn to_monitored_item_create_request(self) -> MonitoredItemCreateRequest {
        // I don't know why I have to do this, but it works
        let node_id: opcua::types::NodeId = self.into();
        let req: opcua::types::MonitoredItemCreateRequest = node_id.into();
        let wrap_req: MonitoredItemCreateRequest = req.into();
        wrap_req
    }
}

#[frb]
pub fn _wrapidentifier(_a: Identifier) {}

#[frb]
pub fn _wrapnodeid(_a: NodeId) {}
