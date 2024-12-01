use anyhow::Result;
use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct UAString(opcua::types::UAString);

impl UAString {
    #[frb(sync, positional)]
    pub fn new(value: String) -> Self {
        Self(value.into())
    }
    #[frb(sync, getter)]
    pub fn value(&self) -> Option<String> {
        self.0.value().clone()
    }
    #[frb(sync, setter)]
    pub fn set_value(&mut self, value: Option<String>) {
        self.0.set_value(value)
    }
    #[frb(sync)]
    /// Returns true if the string is null or empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[frb(sync)]
    /// Returns the length of the string in bytes or -1 for null.
    pub fn len(&self) -> isize {
        self.0.len()
    }
    #[frb(sync)]
    /// Create a null string (not the same as an empty string).
    pub fn null() -> UAString {
        UAString(opcua::types::UAString::null())
    }
    #[frb(sync)]
    /// Test if the string is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
    #[frb(sync)]
    /// This function is meant for use with NumericRange. It creates a substring from this string
    /// from min up to and inclusive of max. Note that min must have an index within the string
    /// but max is allowed to be beyond the end in which case the remainder of the string is
    /// returned (see docs for NumericRange).
    pub fn substring(&self, min: usize, max: usize) -> Result<UAString> {
        self.0
            .substring(min, max)
            .map(UAString)
            .map_err(|_| anyhow::anyhow!("Error extracting substring"))
    }
}

impl From<opcua::types::UAString> for UAString {
    fn from(value: opcua::types::UAString) -> Self {
        UAString(value)
    }
}

impl From<UAString> for opcua::types::UAString {
    fn from(value: UAString) -> Self {
        value.0
    }
}

impl From<&str> for UAString {
    fn from(value: &str) -> Self {
        UAString(opcua::types::UAString::from(value))
    }
}

impl From<String> for UAString {
    fn from(value: String) -> Self {
        UAString(opcua::types::UAString::from(value))
    }
}

impl<'a> From<&'a String> for UAString {
    fn from(value: &'a String) -> Self {
        UAString(opcua::types::UAString::from(value))
    }
}

#[frb(opaque)]
pub type WrapXmlElement = UAString;

#[frb]
pub fn _wrapuastring(_a: UAString) {}

#[frb]
pub fn _wrapxmlelement(_a: WrapXmlElement) {}
