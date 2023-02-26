use bevy::prelude::*;

use super::generic_game_components::{Despawnable, CollideType};

#[derive(Component)]
struct Obstacle;

#[derive(Bundle)]
pub struct ObstacleBundle {
    obstacle: Obstacle,
    dispawnable: Despawnable,
    collide_type: CollideType
}

impl Default for ObstacleBundle {
    fn default() -> Self {
        Self {
            obstacle: Obstacle,
            dispawnable: Despawnable,
            collide_type: CollideType::Obstacle
        }
    }
}