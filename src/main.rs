pub mod game;
pub mod ui_menu;
pub mod models;

use bevy::prelude::*;
use models::app_state::AppState;
use game::{pixel_tank_plugin::PixelTankPlugin, generic_game_plugin::GenericGamePlugin, obstacle_plugin::ObstaclePlugin};
use ui_menu::ui_menu_main_plugin::UiMenuPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
        title: "pixel-tanks".to_string(),
        ..default()
        }),
        ..default()
    }))
    .add_state::<AppState>()
    .add_plugin(UiMenuPlugin)
    .add_plugin(GenericGamePlugin)
    .add_plugin(PixelTankPlugin)
    .add_plugin(ObstaclePlugin)
    .add_startup_system(setup)

    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
