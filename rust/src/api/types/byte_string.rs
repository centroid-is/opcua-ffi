use anyhow::Result;
use flutter_rust_bridge::frb;

#[derive(Clone)]
#[frb(opaque)]
pub struct ByteString(opcua::types::ByteString);

impl ByteString {
    #[frb(sync)]
    /// Create a null string (not the same as an empty string)
    pub fn null() -> ByteString {
        ByteString(opcua::types::ByteString::null())
    }

    #[frb(sync)]
    /// Test if the string is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[frb(sync)]
    // Test if the bytestring has an empty value (not the same as null)
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[frb(sync)]
    /// Test if the string is null or empty
    pub fn is_null_or_empty(&self) -> bool {
        self.0.is_null_or_empty()
    }

    #[frb(sync)]
    /// Creates a byte string from a Base64 encoded string
    pub fn from_base64(data: String) -> Option<ByteString> {
        opcua::types::ByteString::from_base64(&data).map(ByteString)
    }

    #[frb(sync)]
    /// Encodes the bytestring as a Base64 encoded string
    pub fn as_base64(&self) -> String {
        self.0.as_base64()
    }

    #[frb(sync)]
    /// This function is meant for use with NumericRange. It creates a substring from this string
    /// from min up to and inclusive of max. Note that min must have an index within the string
    /// but max is allowed to be beyond the end in which case the remainder of the string is
    /// returned (see docs for NumericRange).
    pub fn substring(&self, min: usize, max: usize) -> Result<ByteString> {
        self.0
            .substring(min, max)
            .map(ByteString)
            .map_err(|_| anyhow::anyhow!("Error extracting substring"))
    }
}

#[frb(ignore)]
impl From<opcua::types::ByteString> for ByteString {
    fn from(value: opcua::types::ByteString) -> Self {
        ByteString(value)
    }
}

#[frb(ignore)]
impl From<ByteString> for opcua::types::ByteString {
    fn from(value: ByteString) -> Self {
        value.0
    }
}

#[frb]
pub fn _wrapbytestring(_a: ByteString) {}
