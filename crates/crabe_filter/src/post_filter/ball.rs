use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;

pub struct BallFilter;

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        world.ball = filter_data.ball.data.clone();
    }
}
