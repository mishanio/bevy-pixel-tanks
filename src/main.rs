use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor{
        title: "pixel-tanks".to_string(),
        ..default()
        },
        ..default()
    }))
    .run();
}
