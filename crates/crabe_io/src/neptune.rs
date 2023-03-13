use crate::communication::WebSocketTransceiver;
use crabe_framework::component::{Component, DecisionComponent, ToolComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use crabe_framework::data::output::{Command, CommandMap};
use clap::Args;
use log::{debug, info};

#[derive(Serialize)]
enum NeptuneMessage {
    World(World),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "requestType", content="payload")]
enum NeptuneRequest {
    Command { id: u8, command: Command }
}

#[derive(Args)]
pub struct NeptuneConfig {
    #[arg(long, default_value_t = 10700)]
    pub neptune_port: u16,
}

pub struct NeptuneServer {
    websocket: WebSocketTransceiver<NeptuneRequest, NeptuneMessage>,
}

impl NeptuneServer {
    pub fn with_config(_common_config: &CommonConfig) -> Self {
        Self {
            websocket: WebSocketTransceiver::spawn(
                SocketAddrV4::new(Ipv4Addr::LOCALHOST, 10700).into(),
            ),
        }
    }
}

impl Component for NeptuneServer {
    fn close(self) {
        self.websocket.close();
    }
}

impl DecisionComponent for NeptuneServer {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData) {
        let mut command_map = CommandMap::new();
        if let Some(request) = self.websocket.receive() {
            match request {
                NeptuneRequest::Command { id, command} => {
                    command_map.insert(id, command);
                }
            }

            info!("command received");
            //debug!("{}", command_map);
        };
        //info!("step");
        //self.websocket.send(NeptuneMessage::World(data.clone()));
        (command_map, ToolData::default())
    }
}
