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

#[derive(Component, Default)]
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
        commands
            .entity(player_ent)
            .insert((
                MovementController { ..default() },
                RigidBody::Dynamic, // affected by gravity/colissions
                ScreenBlock,
                PointLight2d {
                    // for firefly lighting
                    color: Color::srgb(0.6, 0.01, 0.7),
                    range: PLAYER_SPRITE_SIZE * SPRITE_SCALE * 4.,
                    intensity: 1.7,
                    ..default()
                },
            ))
            .with_children(|children| {
                // Spawn the child colliders positioned relative to the rigid body
                children.spawn((
                    PlayerPart,
                    Name::new("PickHead"),
                    Transform::from_xyz(0.0, 10.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    (
                        // Capsule prevents catching on tile edges
                        Collider::capsule(3.0, 13.0),
                        ColliderTransform {
                            translation: Vec2::new(20., -4.),
                            ..Default::default()
                        },
                        CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                        // LockedAxes::ROTATION_LOCKED,
                        Friction::new(0.1),
                        Restitution::new(0.5), // bounciness. 1 = perfectly elastic, 0 = no
                        LinearVelocity::ZERO,  // start stationary
                        CollidingEntities::default(), // track collisions
                        GravityScale(1.0),
                    ),
                ));
                children.spawn((
                    PlayerPart,
                    Name::new("Handle"),
                    Transform::from_xyz(0.0, -4.0, 0.0),
                    (
                        // Capsule prevents catching on tile edges
                        Collider::capsule(2.0, 20.0),
                        ColliderTransform {
                            translation: Vec2::new(20., -4.),
                            ..Default::default()
                        },
                        CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                        // LockedAxes::ROTATION_LOCKED,
                        Friction::new(0.1),
                        Restitution::new(0.3), // bounciness. 1 = perfectly elastic, 0 = no
                        LinearVelocity::ZERO,  // start stationary
                        CollidingEntities::default(), // track collisions
                        GravityScale(1.0),
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
