use flutter_rust_bridge::frb;

// The parser does not handle same name in Variant, Box<Guid> does not work within the Variatn.
#[frb(opaque)]
#[derive(Clone)]
pub struct UAGuid(opcua::types::Guid);

impl UAGuid {
    #[frb(sync)]
    /// Return a null guid, i.e. 00000000-0000-0000-0000-000000000000
    pub fn null() -> UAGuid {
        UAGuid(opcua::types::Guid::null())
    }

    /// Creates a random Guid
    #[frb(sync)]
    pub fn new() -> UAGuid {
        UAGuid(opcua::types::Guid::new())
    }

    /// Returns the bytes of the Guid
    #[frb(sync)]
    pub fn as_bytes(&self) -> [u8; 16] {
        self.0.as_bytes().clone()
    }

    // Creates a guid from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> UAGuid {
        UAGuid(opcua::types::Guid::from_bytes(bytes))
    }
}

impl From<opcua::types::Guid> for UAGuid {
    fn from(value: opcua::types::Guid) -> Self {
        UAGuid(value)
    }
}

impl From<UAGuid> for opcua::types::Guid {
    fn from(value: UAGuid) -> Self {
        value.0
    }
}

#[frb]
pub fn _wrapguid(_a: UAGuid) {}
