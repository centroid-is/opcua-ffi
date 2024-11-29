// Note this is not strictly "types" since it is under "client" mod
use flutter_rust_bridge::frb;
// use opcua::types::{ExtensionObject, MonitoringMode, ReadValueId};
// use std::collections::BTreeSet;

// MonitoredItem is so much private, we cannot access all the fields. TODO make it cloneable make a PR

#[frb(opaque)]
pub struct MonitoredItem {
    /// This is the monitored item's id within the subscription
    id: u32,
    /// Monitored item's handle. Used internally - not modifiable
    client_handle: u32,
    // // The thing that is actually being monitored - the node id, attribute, index, encoding.
    // item_to_monitor: ReadValueId,
    /// Queue size
    queue_size: usize,
    // /// Monitoring mode
    // monitoring_mode: MonitoringMode,
    /// Sampling interval
    sampling_interval: f64,
    // /// Triggered items
    // triggered_items: BTreeSet<u32>,
    /// Whether to discard oldest values on queue overflow
    discard_oldest: bool,
    // /// Active filter
    // filter: ExtensionObject,
}

impl MonitoredItem {
    #[frb(ignore)]
    pub fn new(itm: &opcua::client::MonitoredItem) -> Self {
        Self {
            id: itm.id(),
            client_handle: itm.client_handle(),
            // item_to_monitor: itm.item_to_monitor().clone(),
            queue_size: itm.queue_size(),
            // monitoring_mode: itm.monitoring_mode(),
            sampling_interval: itm.sampling_interval(),
            // triggered_items: itm.triggered_items(),
            discard_oldest: itm.discard_oldest(),
            // filter: itm.filter(),
        }
    }

    #[frb(sync)]
    /// Server assigned ID of the monitored item.
    pub fn id(&self) -> u32 {
        self.id
    }

    #[frb(sync)]
    /// Client assigned handle for the monitored item.
    pub fn client_handle(&self) -> u32 {
        self.client_handle
    }

    // TODO
    // #[frb(sync)]
    // /// Attribute and node ID for the item the monitored item receives notifications for.
    // pub fn item_to_monitor(&self) -> ReadValueId {
    //     self.item_to_monitor.clone()
    // }

    #[frb(sync)]
    /// Sampling interval.
    pub fn sampling_interval(&self) -> f64 {
        self.sampling_interval
    }

    #[frb(sync)]
    /// Queue size on the server.
    pub fn queue_size(&self) -> usize {
        self.queue_size
    }

    #[frb(sync)]
    /// Whether the oldest values are discarded on queue overflow on the server.
    pub fn discard_oldest(&self) -> bool {
        self.discard_oldest
    }
}

impl From<opcua::client::MonitoredItem> for MonitoredItem {
    fn from(value: opcua::client::MonitoredItem) -> Self {
        Self::new(&value)
    }
}

impl From<&opcua::client::MonitoredItem> for MonitoredItem {
    fn from(value: &opcua::client::MonitoredItem) -> Self {
        Self::new(value)
    }
}

// impl From<WrapMonitoredItem> for MonitoredItem {
//     fn from(value: WrapMonitoredItem) -> Self {
//         value.0
//     }
// }

#[frb]
pub fn _monitoreditem(_a: MonitoredItem) {}
