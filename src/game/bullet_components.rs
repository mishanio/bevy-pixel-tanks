use bevy::prelude::*;
use super::components::*;

#[derive(Debug)]
struct BulletSpawnEvent {
    
}

#[derive(Bundle)]
struct BulletBundle {
    movement: Movement,
    dispawnable: Despawnable,
}