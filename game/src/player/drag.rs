use avian2d::prelude::{Forces, WriteRigidBodyForces};
use bevy::{camera::ViewportConversionError, color::palettes::css::WHITE, prelude::*};

use crate::{
    SPRITE_TARGET_PX,
    player::{movement::GRAVITY, physics::PlayerPart},
};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PressPosition::default());
    app.add_systems(Update, (draw_lines, add_player_observer, end_drag));
}

/// length = 3 blocks
const THROW_MAX_LENGTH: f32 = SPRITE_TARGET_PX * 3.;

/// length = 0.5 blocks
const THROW_MIN_LENGTH: f32 = SPRITE_TARGET_PX * 0.5;

const THROW_MIN_SPEED: f32 = SPRITE_TARGET_PX * 1.;
const THROW_MAX_SPEED: f32 = SPRITE_TARGET_PX * 7.;
const THROW_BRACKET_COUNT: i32 = 10;

const BRACKET_SIZE: f32 = (THROW_MAX_LENGTH - THROW_MIN_LENGTH) / THROW_BRACKET_COUNT as f32;

fn get_throw_bracket_nr(distance: f32) -> f32 {
    let distance = distance.clamp(THROW_MIN_LENGTH, THROW_MAX_LENGTH);

    ((distance - THROW_MIN_LENGTH - 0.001) / BRACKET_SIZE).floor()
}

/// gets a [0,1] value and returns a [0,1] value
fn speed_distribution(value: f32) -> f32 {
    value
}

// fn get_throw_speed(distance: f32) -> f32 {
//     let force_prc = get_throw_bracket_nr(distance) / (THROW_BRACKET_COUNT as f32 - 1.);

//     let speed_calculation = force_prc

//     THROW_MIN_SPEED +

// }

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
    println!("I am being pressed");
}

fn end_drag(
    mut ev_drag: MessageReader<Pointer<DragEnd>>,
    mut press_pos: ResMut<PressPosition>,
    mut forces: Query<Forces>,
    camera: Single<&Transform, With<Camera>>,
) {
    if !press_pos.currently_pressed {
        return;
    }

    for drag in ev_drag.read() {
        press_pos.currently_pressed = false;

        println!("drag distance:{}", drag.distance);

        for mut forces in &mut forces {
            let speed: Vec2 = camera.scale.xy() * drag.distance * GRAVITY * Vec2::new(1., -1.);
            forces.apply_linear_impulse_at_point(speed, press_pos.pos);
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
    let dest_reflected = start * 2. - dest;

    // note: gizmos are only loaded for 1 frame.
    gizmos.line_2d(start, dest, WHITE);
    gizmos.line_2d(start, dest_reflected, WHITE);
}
