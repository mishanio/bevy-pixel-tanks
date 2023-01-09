
use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Despawnable;


pub enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

/// Represents movement of entity
/// consists of speed value and direction of movement
#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub direction: MoveDirection,
}

impl Movement {
    pub fn from(direction: MoveDirection) -> Movement {
        Movement { speed: 0.0, direction }
    }

    pub fn to_vec2(&self) -> (f32, f32) {
        match self.direction {
            MoveDirection::Up => (0.0, self.speed),
            MoveDirection::Down => (0.0, -self.speed),
            MoveDirection::Right => (self.speed, 0.0),
            MoveDirection::Left => (-self.speed, 0.0),
        }
    }
}
