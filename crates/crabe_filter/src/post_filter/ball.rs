use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;

pub struct BallFilter;

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let mut ball = filter_data.ball.data.clone();
        if world.data.positive_half == world.team_color {
            ball.position.x = -ball.position.x;
            ball.position.y = -ball.position.y;
        }
        world.ball = Some(ball);
    }
}
