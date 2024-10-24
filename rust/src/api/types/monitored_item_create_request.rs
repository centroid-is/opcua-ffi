use flutter_rust_bridge::frb;
use opcua::types::{MonitoredItemCreateRequest, NodeId};

use super::node_id::WrapNodeId;

#[frb(opaque)]
pub struct WrapMonitoredItemCreateRequest(MonitoredItemCreateRequest);

impl Into<MonitoredItemCreateRequest> for WrapMonitoredItemCreateRequest {
    fn into(self) -> MonitoredItemCreateRequest {
        self.0
    }
}

impl From<MonitoredItemCreateRequest> for WrapMonitoredItemCreateRequest {
    fn from(value: MonitoredItemCreateRequest) -> Self {
        WrapMonitoredItemCreateRequest(value)
    }
}

impl From<WrapNodeId> for WrapMonitoredItemCreateRequest {
    fn from(value: WrapNodeId) -> Self {
        value.into()
    }
}

#[frb]
pub fn _wrapmonitoreditemcreaterequest(_a: WrapMonitoredItemCreateRequest) {}
