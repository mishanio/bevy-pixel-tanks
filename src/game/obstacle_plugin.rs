use bevy::prelude::*;

use crate::models::app_state::AppState;

use super::obstacle_components::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_obstacle));
    }
}


fn setup_obstacle(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: assets_server.load("brick_1.png"),
            ..default()
        })
        .insert(ObstacleBundle::default());
}