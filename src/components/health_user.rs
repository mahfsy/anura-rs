
pub struct HealthUser {
    pub max_hp: f32,
    pub hp: f32,
    alive: bool,
}

impl HealthUser {
    pub fn new(max_hp: f32) -> Self {
        HealthUser {
            max_hp: max_hp,
            hp: max_hp,
            alive: true
        }
    }

    pub fn deal_damage(&mut self, dmg: f32) {
        self.hp -= dmg;
        if self.hp <= 0.0 {
            self.alive = false;
        }
    }

    pub fn heal(&mut self, health: f32) {
        self.hp = (self.hp + health).min(self.max_hp);
    }

    pub fn reset_hp(&mut self) {
        self.hp = self.max_hp;
        self.alive = true;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}
