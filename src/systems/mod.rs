use hecs::World;
use anymap::Map;
use std::vec::Vec;

pub mod auto_pather_system;
pub mod command_system;

pub trait System {
    fn on_start(&mut self, world: &mut World) -> ();
    fn on_tick(&mut self, delta: f32, world: &mut World) -> ();
}
