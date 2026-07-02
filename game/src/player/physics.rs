use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{Player, movement::GRAVITY, physics_bundles::*};

const DAMPING_FACTOR_LINEAR: [f32; 2] = [0.3, 0.1];

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

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec2::new(0.0, GRAVITY)));
    app.add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig {
                enabled: false,
                ..default()
            },
        );
    app.add_systems(
        Update,
        (
            add_avian_body,
            toggle_physics_gizmos,
            apply_directional_damping,
        ),
    );
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

pub fn add_avian_body(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player_ent in new_players.iter() {
        commands
            .entity(player_ent)
            .insert(PlayerBundle::default())
            .with_children(|parent| {
                // Spawn the child colliders positioned relative to the rigid body
                parent.spawn(PickHeadBundle::default()).with_children(|p| {
                    p.spawn(DragHitboxBundle::for_head());
                });
                parent.spawn(HandleBundle::default()).with_children(|p| {
                    p.spawn(DragHitboxBundle::for_handle());
                });
            });
    }
}
