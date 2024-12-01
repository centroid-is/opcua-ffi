use flutter_rust_bridge::frb;
use opcua::types::ExtensionObject;

use crate::api::types::status_code::StatusCode;

#[frb(opaque)]
pub struct MonitoredItemCreateResult {
    pub status_code: StatusCode,
    pub monitored_item_id: u32,
    pub revised_sampling_interval: f64,
    pub revised_queue_size: u32,
    /*pub*/ filter_result: ExtensionObject,
}

impl From<opcua::types::MonitoredItemCreateResult> for MonitoredItemCreateResult {
    fn from(value: opcua::types::MonitoredItemCreateResult) -> Self {
        Self {
            status_code: value.status_code.into(),
            monitored_item_id: value.monitored_item_id,
            revised_sampling_interval: value.revised_sampling_interval,
            revised_queue_size: value.revised_queue_size,
            filter_result: value.filter_result,
        }
    }
}

impl From<MonitoredItemCreateResult> for opcua::types::MonitoredItemCreateResult {
    fn from(value: MonitoredItemCreateResult) -> Self {
        Self {
            status_code: value.status_code.into(),
            monitored_item_id: value.monitored_item_id,
            revised_sampling_interval: value.revised_sampling_interval,
            revised_queue_size: value.revised_queue_size,
            filter_result: value.filter_result,
        }
    }
}
