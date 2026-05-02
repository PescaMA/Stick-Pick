use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_firefly::lights::PointLight2d;

use crate::{
    SPRITE_SCALE,
    player::{
        PLAYER_SPRITE_SIZE, Player,
        movement::{GRAVITY, MovementController, ScreenBlock},
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

#[derive(Component, Default, Clone)]
pub struct PlayerPart;

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
}

fn add_avian_body(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player_ent in new_players.iter() {
        let common_things = (
            PlayerPart,
            LinearVelocity::ZERO,         // start stationary
            CollidingEntities::default(), // track collisions
            GravityScale(1.0),
            Friction::new(0.3), // friction with other colliders
        );

        commands
            .entity(player_ent)
            .insert((
                MovementController { ..default() },
                RigidBody::Dynamic, // affected by gravity/colissions
                SleepingDisabled,
                AngularDamping(DAMPING_FACTOR),
                LinearDamping(DAMPING_FACTOR),
                MaxAngularSpeed(MAX_ANGULAR_SPEED),
                ScreenBlock,
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
                children.spawn((
                    Name::new("PickHead"),
                    Transform::from_xyz(0.0, 10.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    (
                        Mass(HEAD_MASS),
                        // Capsule prevents catching on tile edges
                        Collider::capsule(3.0, 13.0),
                        CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                        Restitution::new(0.5), // bounciness. 1 = perfectly elastic, 0 = no
                        common_things.clone(),
                    ),
                ));
                children.spawn((
                    Name::new("Handle"),
                    Transform::from_xyz(0.0, -4.0, 0.0),
                    (
                        Mass(HANDLE_MASS),
                        // Capsule prevents catching on tile edges
                        Collider::capsule(2.0, 20.0),
                        CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                        Restitution::new(0.3), // bounciness. 1 = perfectly elastic, 0 = no
                        common_things.clone(),
                    ),
                ));
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
