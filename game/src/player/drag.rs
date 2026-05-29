use avian2d::prelude::{Forces, WriteRigidBodyForces};
use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::player::{drag_helper::*, movement::IgnoreSticky, physics_bundles::PlayerPartHitbox};

const COLOR_WEAK: Color = Color::linear_rgb(1., 0.1, 0.1);
const COLOR_STRONG: Color = Color::linear_rgb(0.1, 0.9, 0.1);

#[derive(Resource, Default, Clone)]
pub struct PressPosition {
    pub pos: Vec2,
    pub currently_pressed: bool,
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PressPosition::default());
    // app.add_plugins(drag_simulation::plugin);
    app.add_systems(
        Update,
        (drag_draw_lines, add_player_observer, end_drag, cancel_drag),
    );
}

fn add_player_observer(
    mut commands: Commands,
    new_players: Query<Entity, Added<PlayerPartHitbox>>,
) {
    for player_ent in new_players {
        commands.entity(player_ent).observe(start_drag);
    }
}

fn cancel_drag(buttons: Res<ButtonInput<MouseButton>>, mut press_pos: ResMut<PressPosition>) {
    if buttons.just_pressed(MouseButton::Right) {
        press_pos.currently_pressed = false;
    }
}

fn start_drag(
    ev: On<Pointer<DragStart>>,
    mut press_pos: ResMut<PressPosition>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    press_pos.pos = get_world2d_coords(ev.pointer_location.position, camera_query.clone()).unwrap();
    press_pos.currently_pressed = !press_pos.currently_pressed;
}

pub fn end_drag(
    mut ev_drag: MessageReader<Pointer<DragEnd>>,
    window: Single<&Window>,
    mut press_pos: ResMut<PressPosition>,
    mut forces: Query<Forces>,
    mut player_sticky: Query<&mut IgnoreSticky>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if !press_pos.currently_pressed {
        return;
    }

    if ev_drag.read().len() > 0 {
        press_pos.currently_pressed = false;

        let impulse_pos = press_pos.pos;
        let impulse_velocity = get_throw_distance(window, press_pos.into(), camera_query).unwrap();
        for mut forces in &mut forces {
            forces.apply_linear_impulse_at_point(impulse_velocity, impulse_pos);
        }
        for mut player in player_sticky.iter_mut() {
            player.time = Timer::from_seconds(0.05, TimerMode::Once);
        }

        info!("ACTRUALLY apply {} at {}", impulse_velocity, impulse_pos);
    }
}

// note: NOT using MessageReader<Pointer<Drag>> because it updates only when moving.
fn drag_draw_lines(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
    window: Single<&Window>,
    press_pos: Res<PressPosition>,
) {
    if !press_pos.currently_pressed || window.cursor_position().is_none() {
        return;
    }
    let start = press_pos.pos;

    let cursor_in_screen = window.cursor_position().unwrap();

    let cursor = get_world2d_coords(cursor_in_screen, camera_query.clone()).unwrap();

    let dest = cursor;

    gizmos.line_2d(start, dest, WHITE);

    if (start - dest).length() < THROW_MIN_LENGTH {
        return;
    }

    let normalized_dir = normalize_throw_dir(start - dest);

    let dest_reflected = start + normalized_dir;

    // note: gizmos are only loaded for 1 frame.
    gizmos.line_2d(
        start,
        dest_reflected,
        COLOR_WEAK.mix(
            &COLOR_STRONG,
            get_throw_bracket_prc(normalized_dir.length()),
        ),
    );
}
