use hecs::{World, Entity};
use glam::{Mat3, Vec2};
use std::time::{Duration, Instant};

pub mod components;
pub mod systems;

use components::{
    auto_attacker::{AutoAttacker, AutoAttackerTarget},
    auto_pather::{AutoPather, AutoPatherTarget},
    health_user::HealthUser,
};

use systems::{
    System,
    auto_pather_system::AutoPatherSystem,
    command_system::CommandSystem,
};

pub struct AnuraGame {
    pub world: World,
    pub systems: Vec<Box<dyn System>>,
    is_running : bool,
}

impl AnuraGame {
    pub fn new() -> Self {
        AnuraGame {
            world: World::new(),
            systems: Vec::new(),
            is_running: false,
        }
    }

    pub fn default() -> Self {
        Self::new()
            .with_system(AutoPatherSystem)
            .with_system(CommandSystem::new())
    }

    fn with_system<T : 'static>(mut self, s: T) -> Self 
    where T: System {
        self.systems.push(Box::new(s));
        self
    }

    pub fn run(&mut self) {
        self.is_running = true;
        let num_systems = self.systems.len();

        //on_start
        for i in 0..num_systems {
            match (self.systems.get_mut(i)) {
                Some(system) => system.on_start(&mut self.world),
                None => (),
            }
        }

        let mut now = Instant::now();
        while self.is_running {
            let delta = now.elapsed().as_secs_f32();
            for i in 0..num_systems {
                match (self.systems.get_mut(i)) {
                    Some(system) => system.on_tick(delta, &mut self.world),
                    None => (),
                }
            }
            now = Instant::now();
        }
    }
}
