use hecs::World;
use glam::{swizzles::*, Mat3, Vec2, Vec3};
use crate::components::auto_pather::{
    AutoPather, 
    AutoPatherTarget::*,
};

pub struct AutoPatherSystem;

use crate::systems::System;

impl System for AutoPatherSystem {
    fn on_start(&mut self, world: &mut World) -> () {
    }

    fn on_tick(&mut self, delta: f32, world: &mut World) -> () {
        for (id, (transform, pather)) in world.query::<(&mut Mat3, &mut AutoPather)>().iter() {
            match pather.target {
                FollowTarget(e) => {
    
                },
                LocationTarget(loc) => {
                    let my_loc = transform.col(2).xy();
                    let diff = loc - my_loc;
                    let travel_dist = diff.length();
                    let travel_dir = diff / travel_dist;

                    let move_amount = pather.move_speed * delta;

                    let newloc :Vec2 = if move_amount > travel_dist {
                        pather.target = NoTarget;
                        loc
                    } else {
                        my_loc + (travel_dir * move_amount)
                    };

                    *transform = Mat3::from_cols(
                        transform.col(0),
                        transform.col(1),
                        Vec3::new(newloc.x, newloc.y, 1.0),
                    );
                },
                NoTarget => ()
            }
        }
    }
}
