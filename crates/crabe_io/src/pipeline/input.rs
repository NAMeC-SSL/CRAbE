use crate::league::game_controller::{GameController, GameControllerConfig};
use crate::league::vision::{Vision, VisionConfig};
use clap::Args;
use crabe_framework::component::{Component, InputComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::output::FeedbackMap;

#[derive(Args)]
pub struct InputConfig {
    #[command(flatten)]
    #[command(next_help_heading = "Vision")]
    pub vision_cfg: VisionConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Game Controller")]
    pub gc_cfg: GameControllerConfig,
}

pub trait ReceiverTask {
    fn fetch(&mut self, input: &mut InboundData);
    fn close(&mut self);
}

pub struct InputPipeline {
    receivers: Vec<Box<dyn ReceiverTask>>,
}

impl InputPipeline {
    pub fn with_config(input_cfg: InputConfig, common_cfg: &CommonConfig) -> Self {
        let mut tasks: Vec<Box<dyn ReceiverTask>> = vec![Box::new(Vision::with_config(
            input_cfg.vision_cfg,
            common_cfg,
        ))];

        if common_cfg.gc {
            tasks.push(Box::new(GameController::with_config(input_cfg.gc_cfg)));
        }

        Self { receivers: tasks }
    }
}

impl Component for InputPipeline {
    fn close(mut self) {
        self.receivers.drain(..).for_each(|mut x| x.close());
    }
}

impl InputComponent for InputPipeline {
    fn step(&mut self, _feedback: &mut FeedbackMap) -> InboundData {
        let mut data = InboundData::default();
        self.receivers.iter_mut().for_each(|x| x.fetch(&mut data));
        data
    }
}

#[derive(Args)]
pub struct OutputPipeline {}
