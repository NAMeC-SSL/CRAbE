mod constant;
mod data;
mod filter;
mod post_filter;
mod pre_filter;

use crate::data::FilterData;

use crate::filter::inactive::InactiveFilter;
use crate::filter::passthrough::PassthroughFilter;
use crate::filter::Filter;
use crate::post_filter::ball::BallFilter;
use crate::post_filter::game_controller::GameControllerPostFilter;
use crate::post_filter::geometry::GeometryFilter;
use crate::post_filter::robot::RobotFilter;
use crate::post_filter::PostFilter;
use crate::pre_filter::game_controller::GameControllerPreFilter;
use crate::pre_filter::vision::VisionFilter;
use crate::pre_filter::PreFilter;
use clap::Args;
use crabe_framework::component::{Component, FilterComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::{TeamColor, World};

#[derive(Args)]
pub struct FilterConfig {}

pub struct FilterPipeline {
    pub pre_filters: Vec<Box<dyn PreFilter>>,
    pub filters: Vec<Box<dyn Filter>>,
    pub post_filters: Vec<Box<dyn PostFilter>>,
    pub filter_data: FilterData,
    pub team_color: TeamColor,
}

impl FilterPipeline {
    pub fn with_config(_config: FilterConfig, common_config: &CommonConfig) -> Self {
        let mut pre_filters: Vec<Box<dyn PreFilter>> = vec![Box::new(VisionFilter::new())];
        let filters: Vec<Box<dyn Filter>> = vec![
            Box::new(PassthroughFilter),
            Box::<InactiveFilter>::default(),
        ];
        let mut post_filters: Vec<Box<dyn PostFilter>> = vec![
            Box::new(RobotFilter),
            Box::new(GeometryFilter),
            Box::new(BallFilter),
        ];

        if common_config.gc {
            pre_filters.push(Box::new(GameControllerPreFilter));
            post_filters.push(Box::<GameControllerPostFilter>::default());
        }

        Self {
            pre_filters,
            filters,
            post_filters,
            filter_data: FilterData {
                allies: Default::default(),
                enemies: Default::default(),
                ball: Default::default(),
                geometry: Default::default(),
                referee: Default::default(),
            },
            team_color: if common_config.yellow {
                TeamColor::Yellow
            } else {
                TeamColor::Blue
            },
        }
    }
}

impl Component for FilterPipeline {
    fn close(self) {}
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut inbound_data: InboundData, world: &mut World) {
        self.pre_filters
            .iter_mut()
            .for_each(|f| f.step(&mut inbound_data, &self.team_color, &mut self.filter_data));

        self.filters
            .iter_mut()
            .for_each(|f| f.step(&mut self.filter_data, world));

        self.post_filters
            .iter_mut()
            .for_each(|f| f.step(&self.filter_data, world));
    }
}
