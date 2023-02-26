use bevy::prelude::*;
use super::generic_game_components::*;

#[derive(Debug)]
pub struct BulletSpawnEvent {
    pub spawn_point: Vec3,
    pub movement: Movement,
}

#[derive(Bundle)]
pub struct BulletBundle {
    movement: Movement,
    dispawnable: Despawnable,
    collide_type: CollideType,
}

impl BulletBundle {
    pub fn from_movement(movement: Movement) -> BulletBundle {
        BulletBundle {
            movement: movement,
            dispawnable: Despawnable,
            collide_type: CollideType::Obstacle
        }
    }
}