use crate::communication::WebSocketTransceiver;
use crate::tool::config::ToolConfig;
use crabe_framework::component::{Component, ToolComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Serialize)]
enum ToolMessage {
    World(World),
}

#[derive(Deserialize)]
enum ToolRequest {}

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
    fn step(&mut self, world_data: &World, tool_data: &mut ToolData) -> ToolCommands {
        self.websocket.send(ToolMessage::World(world_data.clone()));
        ToolCommands {}
    }
}
