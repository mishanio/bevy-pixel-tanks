use bevy::prelude::*;
use super::components::*;

#[derive(Debug)]
pub struct BulletSpawnEvent {
    pub spawn_point: Vec3,
    pub movement: Movement,
}

#[derive(Bundle)]
pub struct BulletBundle {
    movement: Movement,
    dispawnable: Despawnable,
}

impl BulletBundle {
    pub fn from_movement(movement: Movement) -> BulletBundle {
        BulletBundle {
            movement: movement,
            dispawnable: Despawnable,
        }
    }
}