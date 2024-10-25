use flutter_rust_bridge::frb;
pub use opcua::types::TimestampsToReturn;

#[frb(mirror(TimestampsToReturn))]
pub enum _TimestampsToReturn {
    Source = 0,
    Server = 1,
    Both = 2,
    Neither = 3,
    Invalid = 4,
}

#[frb]
pub fn _timestampstoreturn(_a: TimestampsToReturn) {}
