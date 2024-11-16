use flutter_rust_bridge::frb;
pub use opcua::types::StatusCode;

#[frb(external)]
impl opcua::types::StatusCode {
    #[frb(sync)]
    pub fn is_good(&self) -> bool {}
    #[frb(sync)]
    pub fn is_bad(&self) -> bool {}
    #[frb(sync)]
    pub fn is_uncertain(&self) -> bool {}
    #[frb(sync)]
    pub fn bits(&self) -> u32 {}
}
