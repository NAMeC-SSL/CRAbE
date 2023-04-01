use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::{TeamColor, World};

pub struct BallFilter;

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let should_flip = match world.data.positive_half {
            TeamColor::Blue => {
                match world.team_color {
                    TeamColor::Blue => {
                        true
                    }
                    TeamColor::Yellow => {
                        false
                    }
                }
            }
            TeamColor::Yellow => {
                match world.team_color {
                    TeamColor::Blue => {
                        false
                    }
                    TeamColor::Yellow => {
                        true
                    }
                }
            }
        };

        if should_flip {
            if let Some(mut ball) = world.ball.clone() {

                ball.position.x *= -1.0;
                ball.velocity.x *= -1.0;
                ball.acceleration.x *= -1.0;
                world.ball = Some(ball);
            }

            for (id, robot) in &mut world.allies_bot {
                robot.pose.position.x *= -1.0;
                robot.velocity.linear.x *= 1.0;
                robot.acceleration.linear.x *= 1.0;
            }

            for (id, robot) in &mut world.enemies_bot {
                robot.pose.position.x *= -1.0;
                robot.velocity.linear.x *= 1.0;
                robot.acceleration.linear.x *= 1.0;
            }
        }


    }
}
