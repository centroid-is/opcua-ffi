use anyhow::Result;
use flutter_rust_bridge::frb;

use opcua::types::DateTime as DT;

#[frb(opaque)]
pub struct WrapDateTime(DT);

impl WrapDateTime {
    #[frb(sync)]
    /// Constructs from the current time
    pub fn now() -> WrapDateTime {
        WrapDateTime(DT::now())
    }

    #[frb(sync)]
    /// Constructs from the current time with an offset
    pub fn now_with_offset(offset: chrono::Duration) -> WrapDateTime {
        WrapDateTime(DT::now_with_offset(offset))
    }

    #[frb(sync)]
    /// Creates a null date time (i.e. the epoch)
    pub fn null() -> WrapDateTime {
        WrapDateTime(DT::null())
    }

    #[frb(sync)]
    /// Tests if the date time is null (i.e. equal to epoch)
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[frb(sync)]
    /// Constructs a date time for the epoch
    pub fn epoch() -> WrapDateTime {
        WrapDateTime(DT::epoch())
    }

    #[frb(sync)]
    /// Constructs a date time for the endtimes
    pub fn endtimes() -> WrapDateTime {
        WrapDateTime(DT::endtimes())
    }

    #[frb(sync)]
    /// Returns the maximum tick value, corresponding to the end of time
    pub fn endtimes_ticks() -> i64 {
        DT::endtimes_ticks()
    }

    #[frb(sync)]
    /// Constructs from a year, month, day
    pub fn ymd(year: u16, month: u16, day: u16) -> WrapDateTime {
        WrapDateTime(DT::ymd(year, month, day))
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
    ) -> WrapDateTime {
        WrapDateTime(DT::ymd_hms(year, month, day, hour, minute, second))
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
    ) -> WrapDateTime {
        WrapDateTime(DT::ymd_hms_nano(
            year, month, day, hour, minute, second, nanos,
        ))
    }

    #[frb(sync)]
    /// Returns an RFC 3339 and ISO 8601 date and time string such as 1996-12-19T16:39:57-08:00.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    // #[frb(sync)]
    // /// Parses an RFC 3339 and ISO 8601 date and time string such as 1996-12-19T16:39:57-08:00, then returns a new DateTime
    // pub fn parse_from_rfc3339(s: String) -> Result<WrapDateTime> {
    //     self.0
    //         .parse_from_rfc3339(&s)
    //         .map(WrapDateTime)
    //         .map_err(|e| anyhow::anyhow!("Error parsing date time: {:?}", e))
    // }

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

impl From<DT> for WrapDateTime {
    fn from(value: DT) -> Self {
        WrapDateTime(value)
    }
}

impl From<WrapDateTime> for DT {
    fn from(value: WrapDateTime) -> Self {
        value.0
    }
}

#[frb]
pub fn _wrapdatetime(_a: WrapDateTime) {}
