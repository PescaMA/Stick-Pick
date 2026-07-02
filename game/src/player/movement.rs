//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use avian2d::prelude::*;
use bevy::prelude::*;
use cli_template::{Pause, menus::Menu};

use crate::{
    AppSystems, PausableSystems, SPRITE_SOURCE_PX,
    level::{
        LevelBoundaries,
        ldtk_entities::{Goal, Sticky},
    },
    player::{Player, physics_bundles::PlayerPart},
};

const DAMPING_FACTOR_LINEAR: [f32; 2] = [0.3, 0.1];

pub const GRAVITY: f32 = -12. * SPRITE_SOURCE_PX;
const GRAVITY_CAP: f32 = -20. * SPRITE_SOURCE_PX;

#[derive(Component, Clone)]
pub struct IgnoreSticky {
    pub time: Timer,
    pub has_hit_sticky: bool,
}

#[derive(Component, Clone)]
pub struct CustomDamping {
    pub x: f32,
    pub y: f32,
}

impl Default for CustomDamping {
    fn default() -> Self {
        Self {
            x: DAMPING_FACTOR_LINEAR[0],
            y: DAMPING_FACTOR_LINEAR[1],
        }
    }
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
            (
                win,
                apply_movement,
                apply_level_block,
                apply_directional_damping,
                gravity_cap,
                handle_sticky_collisions,
            )
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenBlock;

fn apply_level_block(
    mut wrap_query: Query<(&mut Transform, Option<&mut LinearVelocity>), With<ScreenBlock>>,
    level_bound: Res<LevelBoundaries>,
) {
    /// for not getting stuck between edge and block
    const EXTRA_PADDING: f32 = 3.5;

    for (mut transform, velocity_opt) in &mut wrap_query {
        let mut result = transform.translation.xy();
        result.x = result.x.clamp(
            level_bound.origin.x + EXTRA_PADDING,
            level_bound.origin.x + level_bound.length.x - EXTRA_PADDING,
        );

        result.y = result.y.clamp(
            level_bound.origin.y + EXTRA_PADDING,
            level_bound.origin.y + level_bound.length.y - EXTRA_PADDING,
        );

        if let Some(mut vel) = velocity_opt {
            // If clamp changed an axis, reset that component of linear velocity to zero.
            if result.x != transform.translation.x {
                vel.x = 0.0;
            }
            if result.y != transform.translation.y {
                vel.y = 0.0;
            }
        }

        transform.translation = result.extend(transform.translation.z);
    }
}

fn apply_directional_damping(
    mut query: Query<(&mut LinearVelocity, &CustomDamping)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut velocity, damping) in &mut query {
        velocity.x *= (1.0 - damping.x * dt).max(0.0);
        velocity.y *= (1.0 - damping.y * dt).max(0.0);
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
            info!("sticky collision!");

            if !ignore_sticky.time.is_finished() {
                break;
            }
            ignore_sticky.has_hit_sticky = true;
        }
    }
}
