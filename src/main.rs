pub mod pixel_tank_plugin;

use bevy::prelude::*;
use pixel_tank_plugin::PixelTankPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor{
        title: "pixel-tanks".to_string(),
        ..default()
        },
        ..default()
    }))
    .add_plugin(PixelTankPlugin{})
    .add_startup_system(setup)

    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
