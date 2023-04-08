use std::collections::HashMap;

use bevy::prelude::*;

use crate::models::app_state::AppState;

use super::{bullet_components::*, generic_game_components::*};

pub struct GenericGamePlugin;

impl Plugin for GenericGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityRotateEvent>()
            .add_event::<BulletSpawnEvent>()
            .add_system(despawn.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (movement_system, entity_rotate_system, entity_collide_system)
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}

//     .add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
// .add_system(menu.in_set(OnUpdate(AppState::Menu)))
// .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));

fn despawn(mut commands: Commands, q_dispawn: Query<Entity, With<Despawnable>>) {
    for id in q_dispawn.iter() {
        commands.entity(id).despawn_recursive();
    }
}

fn movement_system(mut q_movement: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in q_movement.iter_mut() {
        let (x, y) = movement.to_vec2();
        transform.translation.x += x;
        transform.translation.y += y;
    }
}

fn entity_rotate_system(
    mut entity_rotate_event: EventReader<EntityRotateEvent>,
    mut entity_movement_query: Query<(Entity, &mut Transform), With<Movement>>,
) {
    let rotate_ids: HashMap<Entity, &EntityRotateEvent> = entity_rotate_event
        .iter()
        .map(|event| (event.entity, event))
        .collect();
    for (entity, mut transform) in entity_movement_query.iter_mut() {
        //TODO use directionsW
        if let Some(event) = rotate_ids.get(&entity) {
            let angle = event.direction.angle_between(&event.prev_direction);
            transform.rotate_z(angle);
        }
    }
}

fn entity_collide_system(collide_query: Query<(Entity, &Transform, &Sprite, &CollideType)>) {
    for (entity, transform, sprite, collide_type) in collide_query.iter() {
        //TODO use https://docs.rs/bevy/0.4.0/bevy/sprite/collide_aabb/fn.collide.html
    }
}
