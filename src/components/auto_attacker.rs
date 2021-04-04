use hecs::Entity;

#[derive(PartialEq, Eq)]
pub enum Team {
    Red,
    Blue,
    Neutral
}

pub enum AutoAttackerTarget {
    EntityTarget,
    NoTarget,
}

pub struct AutoAttacker {
    pub target: AutoAttackerTarget,
    pub speed: f32,
    pub range: f32,
    pub animation_time: f32,
    pub bullet_velocity: f32,
}

