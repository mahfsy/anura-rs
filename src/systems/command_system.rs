use hecs::{Entity, World};
use glam::Vec2;
use crate::components::{
    auto_pather::{
        AutoPather, AutoPatherTarget, AutoPatherTarget::*
    },
};
use std::vec::Vec;

use crate::systems::System;

pub enum Command {
    SetAutoPatherTarget(Entity, AutoPatherTarget),
}

use Command::*;

impl Command {
    fn execute(&self, delta: f32, world: &mut World) {
        match self {
            SetAutoPatherTarget(e, t) => set_auto_pather_target(e, t),
        }
    }
}

fn set_auto_pather_target(e: &Entity, t: &AutoPatherTarget) {
    println!("setting the auto pather target!");
}

pub struct CommandSystem {
    queue: Vec<Command>,
}

//adjust game state based on a queue of commands.
//
//theoretically you could reconstruct the entire game
//by storing a list of every command and at what time
//it was executed
impl CommandSystem {
    pub fn new() -> Self {
        CommandSystem {
            queue: Vec::new()
        }
    }
}

impl System for CommandSystem {
    fn on_start(&mut self, world: &mut World) -> () {
    }

    fn on_tick(&mut self, delta: f32, world: &mut World) -> () {
        for command in &self.queue {
            command.execute(delta, world);
        }
    }
}
