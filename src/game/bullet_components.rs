use bevy::prelude::*;
use super::components::*;

#[derive(Debug)]
struct BullteSpawnEvent {
    
}

#[derive(Bundle)]
struct PlayerTankBundle {

    movement: Movement
}