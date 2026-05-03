use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_firefly::lights::PointLight2d;

use crate::{
    SPRITE_SCALE,
    player::{
        PLAYER_SPRITE_SIZE, Player,
        movement::{GRAVITY, IgnoreSticky, MovementController, ScreenBlock},
    },
};

const DAMPING_FACTOR: f32 = 0.5;
const MAX_ANGULAR_SPEED: f32 = 18.;
const LIGHT_COLOR: Color = Color::srgb(0.556, 0.654, 0.238);
const LIGHT_INTENSITY: f32 = 3.2;
const LIGHT_RANGE: f32 = PLAYER_SPRITE_SIZE * SPRITE_SCALE * 3.;

const HEAD_MASS: f32 = 200.;
const HANDLE_MASS: f32 = 100.;
pub const PICKAXE_MASS: f32 = HEAD_MASS + HANDLE_MASS;

#[derive(Component, Default, Clone, PartialEq)]
pub enum PlayerPart {
    #[default]
    PickHead,
    PickHandle,
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec2::new(0.0, GRAVITY)));
    app.add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        );
    app.add_systems(Update, add_avian_body);
    app.add_systems(Update, toggle_physics_gizmos);
}
fn toggle_physics_gizmos(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let ctrl_down =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    if ctrl_down && keyboard.just_pressed(KeyCode::KeyH) {
        let gizmos = config_store.config_mut::<PhysicsGizmos>().0;
        gizmos.enabled = !gizmos.enabled;
    }
}

fn add_avian_body(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player_ent in new_players.iter() {
        let common_things = (
            LinearVelocity::ZERO,         // start stationary
            CollidingEntities::default(), // track collisions
            DebugRender::default()
                .with_collider_color(Color::linear_rgba(0.6, 0.4, 0.4, 0.5))
                .with_aabb_color(Color::WHITE.with_alpha(0.)),
            Friction::new(0.3), // friction with other colliders
        );

        const PICKHEAD_SIZE: Vec2 = Vec2::new(3., 13.);
        const PICKHANDLE_SIZE: Vec2 = Vec2::new(2., 18.);
        const DRAG_PADDING: f32 = 5.;

        commands
            .entity(player_ent)
            .insert((
                GravityScale(1.0),
                MovementController { ..default() },
                RigidBody::Dynamic, // affected by gravity/colissions
                SleepingDisabled,
                AngularDamping(DAMPING_FACTOR),
                LinearDamping(DAMPING_FACTOR),
                MaxAngularSpeed(MAX_ANGULAR_SPEED),
                ScreenBlock,
                IgnoreSticky::default(),
                PointLight2d {
                    // for firefly lighting
                    color: LIGHT_COLOR,
                    range: LIGHT_RANGE,
                    intensity: LIGHT_INTENSITY,
                    ..default()
                },
            ))
            .with_children(|children| {
                // Spawn the child colliders positioned relative to the rigid body
                children
                    .spawn((
                        Name::new("PickHead"),
                        PlayerPart::PickHead,
                        Transform::from_xyz(0.0, 10.0, 0.0)
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                        (
                            Mass(HEAD_MASS),
                            // Capsule prevents catching on tile edges
                            Collider::capsule(PICKHEAD_SIZE.x, PICKHEAD_SIZE.y),
                            CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                            Restitution::new(0.5), // bounciness. 1 = perfectly elastic, 0 = no
                            common_things.clone(),
                        ),
                    ))
                    .with_children(|children1| {
                        // Spawn the child colliders positioned relative to the rigid body
                        children1.spawn((
                            Sensor,
                            Collider::capsule(PICKHEAD_SIZE.x + DRAG_PADDING, PICKHEAD_SIZE.y),
                        ));
                    });
                children
                    .spawn((
                        Name::new("Handle"),
                        PlayerPart::PickHandle,
                        Transform::from_xyz(0.0, -4.0, 0.0),
                        (
                            Mass(HANDLE_MASS),
                            // Capsule prevents catching on tile edges
                            Collider::capsule(PICKHANDLE_SIZE.x, PICKHANDLE_SIZE.y),
                            CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                            Restitution::new(0.3), // bounciness. 1 = perfectly elastic, 0 = no
                            common_things.clone(),
                        ),
                    ))
                    .with_children(|children1| {
                        // Spawn the child colliders positioned relative to the rigid body
                        children1.spawn((
                            Sensor,
                            Collider::capsule(PICKHANDLE_SIZE.x + DRAG_PADDING, PICKHANDLE_SIZE.y),
                        ));
                    });
            });
    }
}

pub fn apply_impulse_on_x(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyX) {
        return;
    }

    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x += 200.0;
        linear_velocity.x += 200.0;

        angular_velocity.0 += 0.5;
    }
}
