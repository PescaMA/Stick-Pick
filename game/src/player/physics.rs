use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_firefly::lights::PointLight2d;

use crate::{
    SPRITE_SCALE,
    player::{
        PLAYER_SPRITE_SIZE, Player,
        movement::{GRAVITY, MovementController, ScreenWrap},
    },
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec2::new(0.0, GRAVITY)));
    app.add_plugins(PhysicsPlugins::default());
    app.add_systems(Update, add_avian_body);
}

fn add_avian_body(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player_ent in new_players.iter() {
        commands.entity(player_ent).insert((
            MovementController { ..default() },
            ScreenWrap,
            (
                RigidBody::Dynamic, // affected by gravity/colissions
                // Capsule prevents catching on tile edges
                Collider::capsule(7.0, 8.0),
                CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
                // LockedAxes::ROTATION_LOCKED,
                Friction::new(0.1),
                Restitution::new(0.3), // bounciness. 1 = perfectly elastic, 0 = no
                LinearVelocity::ZERO,  // start stationary
                CollidingEntities::default(), // track collisions
                GravityScale(1.0),
            ),
            PointLight2d {
                // for firefly lighting
                color: Color::srgb(0.2, 0.0, 1.0),
                range: PLAYER_SPRITE_SIZE * SPRITE_SCALE * 3.,
                intensity: 4.0,
                ..default()
            },
        ));
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
