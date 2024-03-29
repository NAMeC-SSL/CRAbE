pub mod ball;
pub mod geometry;
pub mod robot;

use crate::data::FilterData;
use crabe_framework::data::world::World;

pub trait PostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World);
}
