use std::collections::HashMap;

use bevy::prelude::*;

use crate::models::app_state::AppState;

use super::{bullet_components::{BulletSpawnEvent, BulletBundle}, components::*};

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

#[derive(Debug)]
struct EntityRotateEvent {
    entity: Entity,
    prev_direction: MoveDirection,
    direction: MoveDirection,
}

pub struct PixelTankPlugin;

impl Plugin for PixelTankPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityRotateEvent>()
            .add_event::<BulletSpawnEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement_system)
                    .with_system(player_tank_movement_control_system)
                    .with_system(player_tank_fire_control_system)
                    .with_system(bullet_spawn_system)
                    .with_system(entity_rotate_system),
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

// TODO move to generic pluging
fn despawn(mut commands: Commands, q_dispawn: Query<Entity, With<Despawnable>>) {
    for id in q_dispawn.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn player_tank_movement_control_system(
    keys: Res<Input<KeyCode>>,
    mut entity_rotate_writer: EventWriter<EntityRotateEvent>,
    mut player_tank_query: Query<(Entity, &mut Movement, &Acceleration), With<Player>>,
) {
    let (entity, mut movement, acceleration) = player_tank_query.single_mut();
    let prev_direction = movement.direction.clone();
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
    if prev_direction != movement.direction {
        entity_rotate_writer.send(EntityRotateEvent {
            entity: entity,
            prev_direction: prev_direction,
            direction: movement.direction.clone(),
        })
    }
}

fn player_tank_fire_control_system(
    keys: Res<Input<KeyCode>>,
    mut bullet_spawn_writer: EventWriter<BulletSpawnEvent>,
    mut player_tank_query: Query<(&Movement, &Transform), With<Player>>,
) {
    let (movement, transform) = player_tank_query.single_mut();
    if keys.just_released(KeyCode::Space) {
        bullet_spawn_writer.send(BulletSpawnEvent {
            spawn_point: transform.translation.clone(),
            movement: Movement {
                speed: 5.,
                direction: movement.direction.clone(),
            }
        });
        
    }
}

// TODO move to generic pluging
fn movement_system(mut q_movement: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in q_movement.iter_mut() {
        let (x, y) = movement.to_vec2();
        transform.translation.x += x;
        transform.translation.y += y;
    }
}

// TODO move to generic pluging
fn entity_rotate_system(
    mut entity_rotate_event: EventReader<EntityRotateEvent>,
    mut entity_movement_query: Query<(Entity, &mut Transform), With<Movement>>,
) {
    let rotate_ids: HashMap<Entity, &EntityRotateEvent> = entity_rotate_event
        .iter()
        .map(|event| (event.entity, event))
        .collect();
    for (entity, mut transform) in entity_movement_query.iter_mut() {
        //TODO use directions
        if let Some(event) = rotate_ids.get(&entity) {
            let angle = event.direction.angle_between(&event.prev_direction);
            transform.rotate_z(angle);
        }
    }
}

// TODO move to bullet plugin 
fn bullet_spawn_system(mut commands: Commands, 
    mut bullet_spawn_event: EventReader<BulletSpawnEvent>,
    // mut entity_rotate_writer: EventWriter<EntityRotateEvent>,
    assets_server: Res<AssetServer>) {

    for event in bullet_spawn_event.iter() {
        let angle = MoveDirection::Up.angle_between(&event.movement.direction);
        let mut transform = Transform::from_translation(event.spawn_point).with_scale(Vec3::new(0.5, 0.5, 1.));
        transform.rotate_z(angle);
        commands
        .spawn(SpriteBundle {
            texture: assets_server.load("bullet.png"),
            transform,
            ..default()
        })
        .insert(BulletBundle::from_movement(event.movement.clone()));
        // entity_rotate_writer.send(EntityRotateEvent { entity, prev_direction: MoveDirection::Up, direction: event.movement.direction.clone() })
    }

}
