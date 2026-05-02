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
use cli_template::Pause;

use crate::{AppSystems, PausableSystems, SPRITE_SOURCE_PX, level::LevelBoundaries};

pub const GRAVITY: f32 = -10. * SPRITE_SOURCE_PX;
const GRAVITY_CAP: f32 = -15. * SPRITE_SOURCE_PX;
// const SOLID_LAYER: usize = 0;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (
                apply_movement,
                apply_level_block,
                // collide_wall,
                gravity_cap,
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
        velo.x += velocity.x * time.delta_secs() * -GRAVITY * 1.2;
        velo.y += velocity.y * time.delta_secs() * -GRAVITY * 1.2;

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

fn gravity_cap(mut query: Query<&mut LinearVelocity>, time: Res<Time>) {
    let _ = time.delta_secs();
    for mut linear_velocity in &mut query {
        // Accelerate the entity towards +X at `2.0` units per second squared.
        linear_velocity.y = linear_velocity.y.max(GRAVITY_CAP);
    }
}
