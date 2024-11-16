use flutter_rust_bridge::frb;
pub use opcua::types::UAString;
pub use opcua::types::{ByteString, Guid, Identifier, NodeId, WriteValue};

#[frb]
pub fn _wrapwritevalue(_a: WriteValue) {}
#[frb]
pub fn _wrapnodeid(_a: NodeId) {}
#[frb]
pub fn _wrapidetifier(_a: Identifier) {}
#[frb]
pub fn _wrapguid(_a: Guid) {}
#[frb]
pub fn _wrapbytestring(_a: ByteString) {}
#[frb]
pub fn _wrapuastring(_a: UAString) {}

#[frb(external)]
impl opcua::types::UAString {
    // Non supported signature due to reference
    #[frb(ignore)]
    pub fn value(&self) -> &Option<String> {}
    #[frb(sync)]
    pub fn set_value(&mut self, _value: Option<String>) {}
    #[frb(sync)]
    pub fn is_empty(&self) -> bool {}
    #[frb(sync)]
    pub fn len(&self) -> isize {}
    #[frb(sync)]
    pub fn null() -> opcua::types::UAString {}
    #[frb(sync)]
    pub fn is_null(&self) -> bool {}
    // Non supported signature due to void in return type
    #[frb(ignore)]
    pub fn substring(&self, _min: usize, _max: usize) -> Result<UAString, ()> {}
}

pub trait CustomUAStringFunctions {
    #[frb(sync)]
    fn value_copy(&self) -> Option<String>;
    #[frb(sync)]
    fn substring_nonvoid(&self, min: usize, max: usize) -> Option<opcua::types::UAString>;
}

#[frb]
impl CustomUAStringFunctions for opcua::types::UAString {
    #[frb(sync)]
    fn value_copy(&self) -> Option<String> {
        self.value().clone()
    }
    #[frb(sync)]
    fn substring_nonvoid(&self, min: usize, max: usize) -> Option<opcua::types::UAString> {
        match self.substring(min, max) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

// pub struct SmallTest {
//     pub _somenumber: i64,
// }
//
// impl CustomUAStringFunctions for SmallTest {
//     #[frb(sync)]
//     fn value_copy(&self) -> Option<String> {
//         Some("".to_string())
//     }
//     #[frb(sync)]
//     fn substring_nonvoid(&self, _min: usize, _max: usize) -> Option<i64> {
//         Some(10)
//     }
// }
//
// #[frb]
// pub fn _wrapSmallTest(_a: SmallTest) {}
