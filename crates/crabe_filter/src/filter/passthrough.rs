use crate::data::{FilterData, TrackedBall, TrackedRobot};
use crate::filter::Filter;
use crabe_framework::data::world::{Ball, Pose, Robot, World};
use ringbuffer::RingBufferRead;

fn robot_passthrough<'a, T: 'a + Default>(
    robots: impl Iterator<Item = (&'a u8, &'a mut TrackedRobot<T>)>,
    flip_side: bool
) {
    robots.for_each(|(_id, r)| {
        let last_packet = r.packets.drain().last();
        if let Some(mut packet) = last_packet {
            if flip_side {
                packet.position.x = -packet.position.x;
                packet.position.y = -packet.position.y;
                packet.orientation = (packet.orientation + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI);
            }
            r.data = Robot {
                id: packet.id,
                pose: Pose::new(packet.position, packet.orientation),
                has_ball: false,
                robot_info: T::default(),
                velocity: Default::default(),
                acceleration: Default::default(),
                timestamp: packet.frame_info.t_capture,
            }
        }
    })
}

fn ball_passthrough(ball: &mut TrackedBall, flip_side: bool) {
    let last_packet = ball.packets.drain().last();
    if let Some(mut packet) = last_packet {
        if flip_side {
            packet.position.x = -packet.position.x;
            packet.position.y = -packet.position.y;
        }
        ball.data = Ball {
            position: packet.position,
            timestamp: packet.frame_info.t_capture,
            velocity: Default::default(),
            acceleration: Default::default(),
        }
    }
}

pub struct PassthroughFilter;

impl Filter for PassthroughFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {

        let flip_side = world.team_color == world.data.positive_half;
        robot_passthrough(filter_data.allies.iter_mut(), flip_side);
        robot_passthrough(filter_data.enemies.iter_mut(), flip_side);
        ball_passthrough(&mut filter_data.ball, flip_side);
    }
}
