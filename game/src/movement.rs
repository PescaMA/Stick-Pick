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

use engine::prelude::*;

use crate::{AppSystems, PausableSystems, player::Player};

const STICKY_LAYER: usize = 1;
const SOLID_LAYER: usize = 0;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec2::new(0.0, -800.0)));
    // app.add_plugins(PhysicsPlugins::default());
    app.add_systems(
        Update,
        (
            apply_movement,
            apply_screen_wrap,
            apply_level_wrap,
            collide_sticky,
        )
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

fn collide_sticky(
    mut query: Query<(Entity, &CollidingEntities, &mut GravityScale), With<Player>>,
    names: Query<&Name>,
) {
    for (_, colliding_entities, mut gravity) in query.iter_mut() {
        gravity.0 = 1.0;
        for colide in colliding_entities.iter() {
            if names.get(*colide).is_err() {
                continue;
            }

            let name = names.get(*colide).unwrap().as_str();

            // info!("thy name is {name}. maybe {STICKY_LAYER}");

            if name == STICKY_LAYER.to_string() {
                // info!("touching sticky");
                gravity.0 = 0.0;
            }
        }
    }
}

fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 32.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}

fn apply_level_wrap(
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
    level_bounds: Query<&CameraBounds>,
) {
    for level_bound in &level_bounds {
        let size = level_bound.max - level_bound.min + 2.0 * level_bound.padding;
        for mut transform in &mut wrap_query {
            let mut result = transform.translation.xy();
            if result.y < level_bound.min.y {
                result.y = 100.;
            }
            if result.x < level_bound.min.x {
                result.x += size.x;
            }
            if result.x > level_bound.max.x {
                result.x -= size.x;
            }

            transform.translation = result.extend(transform.translation.z);
        }
    }
}
