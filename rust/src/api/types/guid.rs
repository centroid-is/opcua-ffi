use flutter_rust_bridge::frb;
use opcua::types::Guid;

#[frb(opaque)]
pub struct WrapGuid(Guid);

impl WrapGuid {
    #[frb(sync)]
    /// Return a null guid, i.e. 00000000-0000-0000-0000-000000000000
    pub fn null() -> WrapGuid {
        WrapGuid(Guid::null())
    }

    /// Creates a random Guid
    #[frb(sync)]
    pub fn new() -> WrapGuid {
        WrapGuid(Guid::new())
    }

    /// Returns the bytes of the Guid
    #[frb(sync)]
    pub fn as_bytes(&self) -> [u8; 16] {
        self.0.as_bytes().clone()
    }

    // Creates a guid from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> WrapGuid {
        WrapGuid(Guid::from_bytes(bytes))
    }
}

impl From<Guid> for WrapGuid {
    fn from(value: Guid) -> Self {
        WrapGuid(value)
    }
}

impl From<WrapGuid> for Guid {
    fn from(value: WrapGuid) -> Self {
        value.0
    }
}

#[frb]
pub fn _wrapguid(_a: WrapGuid) {}
