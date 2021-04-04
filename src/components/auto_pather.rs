use hecs::{Entity};
use glam::{Vec2};

pub enum AutoPatherTarget {
    FollowTarget(Entity),
    LocationTarget(Vec2),
    NoTarget,
}

pub struct AutoPather {
    pub target: AutoPatherTarget,
    pub move_speed: f32,
}
