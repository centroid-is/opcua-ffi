// Note this is not strictly "types" since it is under "client" mod
use flutter_rust_bridge::frb;
use opcua::client::MonitoredItem;

#[frb(opaque)]
pub struct WrapMonitoredItem(MonitoredItem);

impl WrapMonitoredItem {
    #[frb(sync)]
    pub fn new(client_handle: u32) -> Self {
        Self(MonitoredItem::new(client_handle))
    }

    #[frb(sync)]
    /// Server assigned ID of the monitored item.
    pub fn id(&self) -> u32 {
        self.0.id()
    }

    #[frb(sync)]
    /// Client assigned handle for the monitored item.
    pub fn client_handle(&self) -> u32 {
        self.0.client_handle()
    }

    // TODO
    // #[frb(sync)]
    // /// Attribute and node ID for the item the monitored item receives notifications for.
    // pub fn item_to_monitor(&self) -> ReadValueId {
    //     &self.0.item_to_monitor()
    // }

    #[frb(sync)]
    /// Sampling interval.
    pub fn sampling_interval(&self) -> f64 {
        self.0.sampling_interval()
    }

    #[frb(sync)]
    /// Queue size on the server.
    pub fn queue_size(&self) -> usize {
        self.0.queue_size()
    }

    #[frb(sync)]
    /// Whether the oldest values are discarded on queue overflow on the server.
    pub fn discard_oldest(&self) -> bool {
        self.0.discard_oldest()
    }
}

impl From<MonitoredItem> for WrapMonitoredItem {
    fn from(value: MonitoredItem) -> Self {
        Self(value)
    }
}

impl From<WrapMonitoredItem> for MonitoredItem {
    fn from(value: WrapMonitoredItem) -> Self {
        value.0
    }
}

#[frb]
pub fn _monitoreditem(_a: WrapMonitoredItem) {}
