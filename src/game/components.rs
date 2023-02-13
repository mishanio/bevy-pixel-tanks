
use bevy::prelude::{Component, Vec2, debug};

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
    pub fn angle_between(&self, direction: &MoveDirection) -> f32 {
    let prev_direction = direction.to_vec2_direction();
    let curent_direction = self.to_vec2_direction();

    let prev_direction_vec = Vec2::new(prev_direction.0, prev_direction.1);
    let curent_direction_vec = Vec2::new(curent_direction.0, curent_direction.1);
    let angle = prev_direction_vec.angle_between(curent_direction_vec);
    debug!(
        "prev_direction_vec {:?}, curent_direction_vec {:?} angle {:?}",
        prev_direction_vec, curent_direction_vec, angle
    );
    return angle;
}
}

/// Represents movement of entity
/// consists of speed value and direction of movement
#[derive(Component, Debug, Clone)]
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
