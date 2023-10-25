use crate::data::{FilterData, TrackedBall, TrackedRobot};
use crate::filter::Filter;
use crabe_framework::data::world::{Ball, Pose, Robot, World};
use ringbuffer::RingBuffer;

fn robot_passthrough<'a, T: 'a + Default>(
    robots: impl Iterator<Item = (&'a u8, &'a mut TrackedRobot<T>)>,
) {
    robots.for_each(|(_id, r)| {
        let last_packet = r.packets.drain().last();
        if let Some(packet) = last_packet {
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

fn ball_passthrough(ball: &mut TrackedBall) {
    let last_packet = ball.packets.drain().last();
    if let Some(packet) = last_packet {
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
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        robot_passthrough(filter_data.allies.iter_mut());
        robot_passthrough(filter_data.enemies.iter_mut());

        let tracked_ball = filter_data.ball.get_or_insert(Default::default());
        ball_passthrough(tracked_ball);
    }
}
