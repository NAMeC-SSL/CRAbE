use crate::communication::WebSocketTransceiver;
use crate::tool::config::ToolConfig;
use crabe_framework::component::{Component, ToolComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ToolMessage {
    world: World,
    data: ToolData,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "requestType", content = "payload")]
enum ToolRequest {
    Commands(#[serde_as(as = "Vec<(_, _)>")] CommandMap),
}

pub struct ToolServer {
    websocket: WebSocketTransceiver<ToolRequest, ToolMessage>,
}

impl ToolServer {
    pub fn with_config(tool_config: ToolConfig, _common_config: &CommonConfig) -> Self {
        Self {
            websocket: WebSocketTransceiver::spawn(
                SocketAddrV4::new(Ipv4Addr::LOCALHOST, tool_config.tool_port).into(),
            ),
        }
    }
}

impl Component for ToolServer {
    fn close(self) {
        self.websocket.close();
    }
}

impl ToolComponent for ToolServer {
    fn step(
        &mut self,
        world_data: &World,
        tool_data: &mut ToolData,
        commands: &mut CommandMap,
    ) -> ToolCommands {
        let msg = ToolMessage {
            data: tool_data.clone(),
            world: world_data.clone(),
        };
        self.websocket.send(msg);
        if let Some(request) = self.websocket.receive() {
            println!("request");
            match request {
                ToolRequest::Commands(tool_commands) => {
                    commands.extend(tool_commands);
                }
            }
        }
        ToolCommands {}
    }
}
