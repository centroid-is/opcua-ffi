// use anyhow::Result;
use flutter_rust_bridge::frb;

// The parser crashes if we use the name DateTime directly
// thread 'main' panicked at /home/dl/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flutter_rust_bridge_codegen-2.6.0/src/library/codegen/parser/mir/parser/ty/concrete.rs:118:38:
// index out of bounds: the len is 0 but the index is 0
// stack backtrace:
//    0:     0x56ccea67c3b5 - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::h1b9dad2a88e955ff
// ...
#[frb(opaque)]
pub struct UADateTime(opcua::types::DateTime);

impl UADateTime {
    #[frb(sync)]
    /// Constructs from the current time
    pub fn now() -> UADateTime {
        UADateTime(opcua::types::DateTime::now())
    }

    #[frb(sync)]
    /// Constructs from the current time with an offset
    pub fn now_with_offset(offset: chrono::Duration) -> UADateTime {
        UADateTime(opcua::types::DateTime::now_with_offset(offset))
    }

    #[frb(sync)]
    /// Creates a null date time (i.e. the epoch)
    pub fn null() -> UADateTime {
        UADateTime(opcua::types::DateTime::null())
    }

    #[frb(sync)]
    /// Tests if the date time is null (i.e. equal to epoch)
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[frb(sync)]
    /// Constructs a date time for the epoch
    pub fn epoch() -> UADateTime {
        UADateTime(opcua::types::DateTime::epoch())
    }

    #[frb(sync)]
    /// Constructs a date time for the endtimes
    pub fn endtimes() -> UADateTime {
        UADateTime(opcua::types::DateTime::endtimes())
    }

    #[frb(sync)]
    /// Returns the maximum tick value, corresponding to the end of time
    pub fn endtimes_ticks() -> i64 {
        opcua::types::DateTime::endtimes_ticks()
    }

    #[frb(sync)]
    /// Constructs from a year, month, day
    pub fn ymd(year: u16, month: u16, day: u16) -> UADateTime {
        UADateTime(opcua::types::DateTime::ymd(year, month, day))
    }

    #[frb(sync)]
    /// Constructs from a year, month, day, hour, minute, second
    pub fn ymd_hms(
        year: u16,
        month: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
    ) -> UADateTime {
        UADateTime(opcua::types::DateTime::ymd_hms(
            year, month, day, hour, minute, second,
        ))
    }

    #[frb(sync)]
    /// Constructs from a year, month, day, hour, minute, second, nanosecond
    pub fn ymd_hms_nano(
        year: u16,
        month: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        nanos: u32,
    ) -> UADateTime {
        UADateTime(opcua::types::DateTime::ymd_hms_nano(
            year, month, day, hour, minute, second, nanos,
        ))
    }

    #[frb(sync)]
    /// Returns an RFC 3339 and ISO 8601 date and time string such as 1996-12-19T16:39:57-08:00.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    #[frb(sync)]
    /// Returns the time in ticks, of 100 nanosecond intervals
    pub fn ticks(&self) -> i64 {
        self.0.ticks()
    }

    #[frb(sync)]
    /// To checked ticks. Function returns 0 or MAX_INT64
    /// if date exceeds valid OPC UA range
    pub fn checked_ticks(&self) -> i64 {
        self.0.checked_ticks()
    }

    #[frb(sync)]
    /// Time as chrono
    pub fn as_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        self.0.as_chrono()
    }
}

impl From<opcua::types::DateTime> for UADateTime {
    fn from(value: opcua::types::DateTime) -> Self {
        UADateTime(value)
    }
}

impl From<UADateTime> for opcua::types::DateTime {
    fn from(value: UADateTime) -> Self {
        value.0
    }
}

#[frb]
pub fn _datetime(_a: UADateTime) {}
