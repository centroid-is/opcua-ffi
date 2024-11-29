use flutter_rust_bridge::frb;

use std::str::FromStr;

#[derive(Clone)]
#[frb(opaque)]
pub struct StatusCode(opcua::types::StatusCode);

impl StatusCode {
    #[frb(sync)]
    pub fn name(self) -> String {
        self.0.name().to_string()
    }
    #[frb(sync)]
    pub fn description(self) -> String {
        self.0.description().to_string()
    }
    #[frb(sync, positional)]
    pub fn from_u32(code: u32) -> Option<StatusCode> {
        opcua::types::StatusCode::from_u32(code).map(StatusCode)
    }
    #[frb(sync, positional)]
    pub fn from_str(s: String) -> Option<StatusCode> {
        opcua::types::StatusCode::from_str(&s).map(StatusCode).ok()
    }
}

impl From<opcua::types::StatusCode> for StatusCode {
    fn from(value: opcua::types::StatusCode) -> Self {
        StatusCode(value)
    }
}

impl From<StatusCode> for opcua::types::StatusCode {
    fn from(value: StatusCode) -> Self {
        value.0
    }
}

// impl fmt::Display for WrapStatusCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let code: StatusCode = self.into();
//         write!(f, "{}", code)
//     }
// }

#[frb]
pub fn _wrapstatuscode(_a: StatusCode) {}
