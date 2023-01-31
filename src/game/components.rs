
use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Despawnable;


#[derive(Clone, PartialEq, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

impl MoveDirection {

    pub fn to_vec2_direction(&self) -> (f32, f32) {
        self.to_vec2(1.0)
    }

    pub fn to_vec2(&self, speed: f32) -> (f32, f32) {
        match self {
            MoveDirection::Up => (0.0, 1.0 * speed),
            MoveDirection::Down => (0.0, -1.0 * speed),
            MoveDirection::Right => (1.0 * speed, 0.0),
            MoveDirection::Left => (-1.0 * speed, 0.0),
        }
    }
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
        self.direction.to_vec2(self.speed)
    }
}
