use flutter_rust_bridge::frb;

// use super::node_id::WrapNodeId;

#[derive(Debug)]
#[frb(opaque)]
pub struct MonitoredItemCreateRequest(opcua::types::MonitoredItemCreateRequest);

impl Into<opcua::types::MonitoredItemCreateRequest> for MonitoredItemCreateRequest {
    fn into(self) -> opcua::types::MonitoredItemCreateRequest {
        self.0
    }
}

impl From<opcua::types::MonitoredItemCreateRequest> for MonitoredItemCreateRequest {
    fn from(value: opcua::types::MonitoredItemCreateRequest) -> Self {
        MonitoredItemCreateRequest(value)
    }
}

// Dont remember why this was here
// todo do we need this?
// #[frb(sync)]
// impl From<WrapNodeId> for opcua::types::MonitoredItemCreateRequest {
//     #[frb(sync)]
//     fn from(value: WrapNodeId) -> Self {
//         let node_id: opcua::types::NodeId = value.into();
//         let monitoring_item: opcua::types::MonitoredItemCreateRequest = node_id.into();
//         monitoring_item
//     }
// }

#[frb]
pub fn _monitoreditemcreaterequest(_a: MonitoredItemCreateRequest) {}
