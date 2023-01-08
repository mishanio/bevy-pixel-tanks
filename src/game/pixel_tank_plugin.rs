use bevy::prelude::*;

use crate::models::app_state::AppState;

use super::components::Despawnable;

pub struct PixelTankPlugin;

impl Plugin for PixelTankPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn));
    }
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: assets_server.load("tank pixel.png"),
            ..default()
        })
        .insert(Despawnable);
}

fn despawn(mut commands: Commands, q_dispawn: Query<Entity, With<Despawnable>>) {
    for id in q_dispawn.iter() {
        commands.entity(id).despawn_recursive();
    }
}
