use bevy::prelude::*;

use crate::models::app_state::AppState;

use super::components::*;

const Z_TANK: f32 = 1.0;

#[derive(Component)]
struct Acceleration {
    speed: f32,
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerTankBundle {
    player: Player,
    dispawnable: Despawnable,
    movement: Movement,
    acceleration: Acceleration,
}

impl Default for PlayerTankBundle {
    fn default() -> Self {
        Self {
            player: Player,
            dispawnable: Despawnable,
            movement: Movement::from(MoveDirection::Up),
            acceleration: Acceleration { speed: 1.0 },
        }
    }
}

pub struct PixelTankPlugin;

impl Plugin for PixelTankPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement_system)
                    .with_system(player_tank_control_system),
            );
    }
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: assets_server.load("tank pixel.png"),
            ..default()
        })
        .insert(PlayerTankBundle::default());
}

fn despawn(mut commands: Commands, q_dispawn: Query<Entity, With<Despawnable>>) {
    for id in q_dispawn.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn player_tank_control_system(
    keys: Res<Input<KeyCode>>,
    mut player_tank_query: Query<(&mut Movement, &Acceleration), With<Player>>,
) {
    let (mut movement, acceleration) = player_tank_query.single_mut();
    if keys.pressed(KeyCode::Up) {
        movement.speed = acceleration.speed;
        movement.direction = MoveDirection::Up;
    } else if keys.pressed(KeyCode::Down) {
        movement.speed = acceleration.speed;
        movement.direction = MoveDirection::Down;
    } else if keys.pressed(KeyCode::Right) {
        movement.speed = acceleration.speed;
        movement.direction = MoveDirection::Right;
    } else if keys.pressed(KeyCode::Left) {
        movement.speed = acceleration.speed;
        movement.direction = MoveDirection::Left;
    } else {
        movement.speed = 0.0;
    }
}

fn movement_system(mut q_movement: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in q_movement.iter_mut() {
        let (x, y) = movement.to_vec2();
        transform.translation.x += x;
        transform.translation.y += y;
    }
}
