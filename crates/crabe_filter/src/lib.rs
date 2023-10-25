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
use crate::post_filter::geometry::GeometryFilter;
use crate::post_filter::robot::RobotFilter;
use crate::post_filter::PostFilter;
use crate::pre_filter::vision::VisionFilter;
use crate::pre_filter::PreFilter;
use clap::{Args, ValueEnum};
use crabe_framework::component::{Component, FilterComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::{TeamColor, World};
use filter::velocity_acceleration::VelocityAccelerationFilter;
use crate::filter::field_side::FieldSideFilter;

#[derive(Args)]
pub struct FilterConfig {
    #[arg(long)]
    field_side: Option<FieldSide>
}

#[derive(Debug, ValueEnum, Clone)]
pub enum FieldSide {
    Positive,
    Negative
}

pub struct FilterPipeline {
    pub pre_filters: Vec<Box<dyn PreFilter>>,
    pub filters: Vec<Box<dyn Filter>>,
    pub post_filters: Vec<Box<dyn PostFilter>>,
    pub filter_data: FilterData,
    pub team_color: TeamColor,
}

impl FilterPipeline {
    pub fn with_config(config: FilterConfig, common_config: &CommonConfig) -> Self {
        let mut pre_filters: Vec<Box<dyn PreFilter>> = vec![Box::new(VisionFilter::new())];
        let mut filters: Vec<Box<dyn Filter>> = vec![
            Box::new(PassthroughFilter),
            Box::new(VelocityAccelerationFilter),
            Box::<InactiveFilter>::default(),
        ];

        if let Some(field_side) = config.field_side {
            filters.push(Box::new(FieldSideFilter::new(field_side)))
        }

        let mut post_filters: Vec<Box<dyn PostFilter>> = vec![
            Box::new(RobotFilter),
            Box::new(GeometryFilter),
            Box::new(BallFilter),
        ];

        Self {
            pre_filters,
            filters,
            post_filters,
            filter_data: FilterData {
                allies: Default::default(),
                enemies: Default::default(),
                ball: Default::default(),
                geometry: Default::default(),
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
    fn step(&mut self, inbound_data: InboundData, world: &mut World) {
        self.pre_filters
            .iter_mut()
            .for_each(|f| f.step(&inbound_data, &self.team_color, &mut self.filter_data));

        self.filters
            .iter_mut()
            .for_each(|f| f.step(&mut self.filter_data, world));

        self.post_filters
            .iter_mut()
            .for_each(|f| f.step(&self.filter_data, world));
    }
}
