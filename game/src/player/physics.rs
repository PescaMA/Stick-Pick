use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{Player, movement::GRAVITY, physics_bundles::*};

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
