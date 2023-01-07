use bevy::prelude::*;



pub struct PixelTankPlugin {}

impl Plugin for PixelTankPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer> ) {
    commands.spawn(SpriteBundle{
        texture: assets_server.load("tank pixel.png"),
        ..default()
    });
    
}