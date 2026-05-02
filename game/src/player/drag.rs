use avian2d::prelude::{Forces, WriteRigidBodyForces};
use bevy::{camera::ViewportConversionError, color::palettes::css::WHITE, prelude::*};

use crate::{
    SPRITE_SOURCE_PX,
    player::{
        movement::GRAVITY,
        physics::{PICKAXE_MASS, PlayerPart},
    },
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PressPosition::default());
    app.add_systems(Update, (draw_lines, add_player_observer, end_drag));
}

/// length in blocks
const THROW_MAX_LENGTH: f32 = SPRITE_SOURCE_PX * 4.;
const THROW_MIN_LENGTH: f32 = SPRITE_SOURCE_PX * 0.5;

const THROW_MIN_SPEED: f32 = SPRITE_SOURCE_PX * 1.;
const THROW_MAX_SPEED: f32 = SPRITE_SOURCE_PX * 7.;
const THROW_BRACKET_COUNT: i32 = 6;

const BRACKET_SIZE: f32 = (THROW_MAX_LENGTH - THROW_MIN_LENGTH) / THROW_BRACKET_COUNT as f32;

/// returns value between 0 and THROW_BRACKET_COUNT - 1
fn get_throw_bracket_nr(distance: f32) -> f32 {
    // returns bracket nr after split in [start,end). so max value must be a tad bit smaller
    let distance = distance.clamp(THROW_MIN_LENGTH, THROW_MAX_LENGTH - 0.001);

    ((distance - THROW_MIN_LENGTH) / BRACKET_SIZE).floor()
}

/// gets a [0,1] value and returns a [0,1] value. used for smoother throwing power
fn speed_distribution(value: f32) -> f32 {
    value
}

fn get_throw_speed(distance: f32) -> f32 {
    if THROW_BRACKET_COUNT < 2 {
        return (THROW_MIN_SPEED + THROW_MAX_SPEED) / 2.;
    }
    let force_prc = get_throw_bracket_nr(distance) / (THROW_BRACKET_COUNT as f32 - 1.);

    THROW_MIN_SPEED + speed_distribution(force_prc) * (THROW_MAX_SPEED - THROW_MIN_SPEED)
}

fn normalize_throw_dir(dest: Vec2) -> Vec2 {
    match dest.try_normalize() {
        None => dest,
        Some(dir) => dir * dest.length().clamp(THROW_MIN_LENGTH, THROW_MAX_LENGTH),
    }
}

#[derive(Resource, Default)]
struct PressPosition {
    pos: Vec2,
    currently_pressed: bool,
}

fn add_player_observer(mut commands: Commands, new_players: Query<Entity, Added<PlayerPart>>) {
    for player_ent in new_players {
        commands.entity(player_ent).observe(start_drag);
    }
}

/// Converts a screen-space position to world-space (2D) using a camera's transform.
///
/// The system that calls this function must have a parameter:
///
/// `camera_query: Single<(&Camera, &GlobalTransform)>,`
///
/// and then call this function like:
///
/// `get_world2d_coords(ev.distance, camera_query.clone()).unwrap();`
///
fn get_world2d_coords(
    pos: Vec2,
    camera_query: (&Camera, &GlobalTransform),
) -> Result<Vec2, ViewportConversionError> {
    let (camera, camera_transform) = camera_query;
    camera.viewport_to_world_2d(camera_transform, pos)
}

fn start_drag(
    ev: On<Pointer<Press>>,
    mut press_pos: ResMut<PressPosition>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    press_pos.pos = get_world2d_coords(ev.pointer_location.position, camera_query.clone()).unwrap();
    press_pos.currently_pressed = true;
}

fn end_drag(
    mut ev_drag: MessageReader<Pointer<DragEnd>>,
    window: Single<&Window>,
    mut press_pos: ResMut<PressPosition>,
    mut forces: Query<Forces>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if !press_pos.currently_pressed {
        return;
    }

    for _ in ev_drag.read() {
        press_pos.currently_pressed = false;

        if window.cursor_position().is_none() {
            return;
        }

        let cursor_in_screen = window.cursor_position().unwrap();
        let cursor = get_world2d_coords(cursor_in_screen, camera_query.clone()).unwrap();

        let dist = normalize_throw_dir(cursor - press_pos.pos);

        for mut forces in &mut forces {
            let velocity_squared =
                GRAVITY * 2.0 * get_throw_speed(dist.length()) * dist.normalize_or_zero();

            forces.apply_linear_impulse_at_point(
                PICKAXE_MASS * velocity_squared.map(|c| c.abs().sqrt() * c.signum()),
                press_pos.pos,
            );
        }
    }
}

// note: NOT using MessageReader<Pointer<Drag>> because it updates only when moving.

fn draw_lines(
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
    let normalized_dir = normalize_throw_dir(start - dest);

    let dest_reflected = start + normalized_dir;

    // note: gizmos are only loaded for 1 frame.
    gizmos.line_2d(start, dest, WHITE);
    gizmos.line_2d(start, dest_reflected, WHITE);
}
