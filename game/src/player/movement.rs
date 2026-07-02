use avian2d::prelude::*;
use bevy::prelude::*;
use cli_template::{Pause, menus::Menu};

use crate::{
    AppSystems, PausableSystems, SPRITE_SOURCE_PX,
    level::ldtk_entities::{Goal, Sticky},
    player::{Player, physics_bundles::PlayerPart},
};

pub const GRAVITY: f32 = -12. * SPRITE_SOURCE_PX;
const GRAVITY_CAP: f32 = -20. * SPRITE_SOURCE_PX;

#[derive(Component, Clone)]
pub struct IgnoreSticky {
    pub time: Timer,
    pub has_hit_sticky: bool,
}

impl IgnoreSticky {
    pub fn reset(&mut self) {
        self.time = Timer::from_seconds(0.05, TimerMode::Once);
        self.has_hit_sticky = false;
    }
}

impl Default for IgnoreSticky {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(0., TimerMode::Once),
            has_hit_sticky: false,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (win, apply_movement, gravity_cap, handle_sticky_collisions)
                .chain()
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
            pause_physics.run_if(in_state(Pause(true))),
            unpause_physics.run_if(in_state(Pause(false))),
        ),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    pub intent: Vec2,
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}
fn unpause_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut LinearVelocity)>,
) {
    for (controller, mut velo) in &mut movement_query {
        let velocity = controller.intent;
        velo.x += velocity.x * time.delta_secs() * -GRAVITY * 2.;
        velo.y += velocity.y * time.delta_secs() * -GRAVITY * 2.;

        // info!("speed: {}", velo.y);
    }
}

fn gravity_cap(mut query: Query<&mut LinearVelocity>) {
    for mut linear_velocity in &mut query {
        linear_velocity.y = linear_velocity.y.max(GRAVITY_CAP);
    }
}

pub(crate) fn win(
    head_collisions: Query<&CollidingEntities, With<PlayerPart>>,
    goals: Query<(), With<Goal>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    for collision in head_collisions {
        for col_entity in collision.iter() {
            if goals.get(*col_entity).is_ok() {
                next_menu.set(Menu::Win);
                return;
            }
        }
    }
}

pub(crate) fn handle_sticky_collisions(
    head_collisions: Query<(&PlayerPart, &ChildOf, &CollidingEntities)>,
    mut parent_speed: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut GravityScale,
            &mut IgnoreSticky,
        ),
        With<Player>,
    >,
    sticky: Query<(), With<Sticky>>,
    time: Res<Time>,
    // mut commands: Commands,
) {
    for (player, parent, colliding_entities) in head_collisions {
        if *player != PlayerPart::PickHead {
            continue;
        }

        let Ok((mut velocity, mut angular_velocity, mut grav_scale, mut ignore_sticky)) =
            parent_speed.get_mut(parent.parent())
        else {
            break;
        };

        ignore_sticky.time.tick(time.delta());
        if ignore_sticky.time.just_finished() {
            grav_scale.0 = 1.;
            continue;
        }

        // stop speed after collission
        if colliding_entities.is_empty() && ignore_sticky.has_hit_sticky {
            *velocity = LinearVelocity::ZERO;
            *angular_velocity = AngularVelocity::ZERO;
            grav_scale.0 = 0.;
        }

        for entity in colliding_entities.iter() {
            if sticky.get(*entity).is_err() {
                continue;
            }
            // info!("sticky collision!");

            if !ignore_sticky.time.is_finished() {
                break;
            }

            if !ignore_sticky.has_hit_sticky {
                ignore_sticky.has_hit_sticky = true;

                // *velocity = LinearVelocity(-velocity.0);
                // *angular_velocity = AngularVelocity(-angular_velocity.0);
            }
        }
    }
}
