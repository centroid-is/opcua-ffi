use flutter_rust_bridge::frb;

use opcua::types::StatusCode;
use std::str::FromStr;

#[derive(Clone)]
#[frb(opaque)]
pub struct WrapStatusCode(StatusCode);

impl WrapStatusCode {
    #[frb(sync)]
    pub fn name(self) -> String {
        self.0.name().to_string()
    }
    #[frb(sync)]
    pub fn description(self) -> String {
        self.0.description().to_string()
    }
    #[frb(sync, positional)]
    pub fn from_u32(code: u32) -> Option<WrapStatusCode> {
        StatusCode::from_u32(code).map(WrapStatusCode)
    }
    #[frb(sync, positional)]
    pub fn from_str(s: String) -> Option<WrapStatusCode> {
        StatusCode::from_str(&s).map(WrapStatusCode).ok()
    }
}

impl From<StatusCode> for WrapStatusCode {
    fn from(value: StatusCode) -> Self {
        WrapStatusCode(value)
    }
}

impl From<WrapStatusCode> for StatusCode {
    fn from(value: WrapStatusCode) -> Self {
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
pub fn _wrapstatuscode(_a: WrapStatusCode) {}
