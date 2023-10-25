use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::World;

pub struct BallFilter;

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let ball = filter_data.ball.as_ref().map(|tracked_ball| {
            let mut ball = tracked_ball.data.clone();
            if world.data.positive_half == world.team_color {
                ball.position.x = -ball.position.x;
                ball.position.y = -ball.position.y;
            }

            ball
        });

        if ball.is_some() {
            world.ball = ball;
        } 
    }
}
