use avian2d::collision::collider::CollidingEntities;
use avian2d::prelude::*;
use bevy::prelude::*;
use cli_template::{PausableSystems, audio::sound_effect};

use crate::{
    drag::{PressPosition, drag_helper::get_throw_distance, end_drag},
    level::ldtk_entities::{Sticky, Wall},
    player::{
        Player, PlayerAssets,
        movement::{IgnoreSticky, handle_sticky_collisions},
        physics_bundles::PlayerPart,
    },
};

const MIN_LINEAR_SPEED_FOR_SOUND: f32 = 30.;
// const MIN_ANGULAR_SPEED_FOR_SOUND: f32 = 300.;

#[derive(Component, Default)]
pub struct PlayerActions {
    is_thrown: bool,
    hit_sticky: bool,
    hit_non_sticky: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (add_throw_action).before(end_drag),
            add_collision_action.before(handle_sticky_collisions),
            add_player_actions,
            trigger_player_sounds,
        )
            .in_set(PausableSystems),
    );
}

fn add_player_actions(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player_ent in new_players.iter() {
        commands.entity(player_ent).insert(PlayerActions::default());
    }
}

fn add_throw_action(
    mut ev_drag: MessageReader<Pointer<DragEnd>>,
    window: Single<&Window>,
    press_pos: Res<PressPosition>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    actions: Query<&mut PlayerActions>,
) {
    if !press_pos.currently_pressed {
        return;
    }

    if ev_drag.read().len() > 0 {
        let impulse_velocity = get_throw_distance(window, press_pos.into(), camera_query).unwrap();

        if impulse_velocity.length() <= 0.0 {
            return;
        }

        for mut action in actions {
            action.is_thrown = true;
        }
    }
}

fn add_collision_action(
    head_collisions: Query<(&PlayerPart, &ChildOf, &CollidingEntities)>,
    mut parent_stick: Query<
        (&mut LinearVelocity, &mut AngularVelocity, &IgnoreSticky),
        With<Player>,
    >,
    sticky: Query<(), With<Sticky>>,
    non_sticky: Query<(), (With<Wall>, Without<Sticky>)>,
    mut actions: Query<&mut PlayerActions>,
) {
    for (player, parent, colliding_entities) in head_collisions {
        let Ok((linear_velocity, _angular_velocity, ignore_sticky)) =
            parent_stick.get_mut(parent.parent())
        else {
            break;
        };

        for entity in colliding_entities.iter() {
            if sticky.get(*entity).is_ok()
                && *player == PlayerPart::PickHead
                && !ignore_sticky.has_hit_sticky
            {
                for mut action in &mut actions {
                    action.hit_sticky = true;
                }
            }

            // info!("angular: {}", _angular_velocity.0.abs().to_degrees());
            // info!("linear: {}", linear_velocity.length());

            if linear_velocity.length() < MIN_LINEAR_SPEED_FOR_SOUND {
                continue;
            }

            if non_sticky.get(*entity).is_ok() {
                for mut action in &mut actions {
                    action.hit_non_sticky = true;
                }
            }
        }
    }
}

fn trigger_player_sounds(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut actions: Query<&mut PlayerActions>,
) {
    for mut action in &mut actions {
        if action.is_thrown {
            commands.spawn(sound_effect(player_assets.get_random_throw_sound()));

            action.is_thrown = false;
        }
        if action.hit_sticky {
            commands.spawn(sound_effect(player_assets.get_random_sticky_sound()));

            action.hit_sticky = false;
        }
        if action.hit_non_sticky {
            commands.spawn(sound_effect(player_assets.get_random_non_sticky_sound()));

            action.hit_non_sticky = false;
        }
    }
}
