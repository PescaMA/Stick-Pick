use crate::{
    SPRITE_SOURCE_PX,
    drag::PressPosition,
    player::{movement::GRAVITY, physics_bundles::PICKAXE_MASS},
};
use bevy::{camera::ViewportConversionError, prelude::*};

/// length in blocks
const THROW_MAX_LENGTH: f32 = SPRITE_SOURCE_PX * 4.;
pub const THROW_MIN_LENGTH: f32 = SPRITE_SOURCE_PX * 0.5;

const THROW_MIN_SPEED: f32 = SPRITE_SOURCE_PX * 0.1;
const THROW_MAX_SPEED: f32 = SPRITE_SOURCE_PX * 7.;
const THROW_BRACKET_COUNT: i32 = 6;

const BRACKET_SIZE: f32 = (THROW_MAX_LENGTH - THROW_MIN_LENGTH) / THROW_BRACKET_COUNT as f32;

/// returns value between 0 and THROW_BRACKET_COUNT - 1
pub fn get_throw_bracket_nr(distance: f32) -> f32 {
    // returns bracket nr after split in [start,end). so max value must be a tad bit smaller
    let distance = distance.clamp(THROW_MIN_LENGTH, THROW_MAX_LENGTH - 0.001);

    ((distance - THROW_MIN_LENGTH) / BRACKET_SIZE).floor()
}
/// returns value between 0 and 1
pub fn get_throw_bracket_prc(distance: f32) -> f32 {
    if THROW_BRACKET_COUNT < 2 {
        return 0.;
    }

    get_throw_bracket_nr(distance) / (THROW_BRACKET_COUNT as f32 - 1.)
}

pub fn get_throw_discrete_distance(distance: f32) -> f32 {
    THROW_MIN_LENGTH + get_throw_bracket_prc(distance) * (THROW_MAX_LENGTH - THROW_MIN_LENGTH)
}

/// gets a [0,1] value and returns a [0,1] value. used for smoother throwing power
fn speed_distribution(value: f32) -> f32 {
    value * value
}

pub fn get_throw_speed(distance: f32) -> f32 {
    if THROW_BRACKET_COUNT < 2 {
        return (THROW_MIN_SPEED + THROW_MAX_SPEED) / 2.;
    }
    let force_prc = get_throw_bracket_prc(distance);

    THROW_MIN_SPEED + speed_distribution(force_prc) * (THROW_MAX_SPEED - THROW_MIN_SPEED)
}

pub fn normalize_throw_dir(dest: Vec2) -> Vec2 {
    if dest.length() < THROW_MIN_LENGTH {
        return Vec2::ZERO;
    }

    match dest.try_normalize() {
        None => dest,
        Some(dir) => dir * get_throw_discrete_distance(dest.length()),
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
pub fn get_world2d_coords(
    pos: Vec2,
    camera_query: (&Camera, &GlobalTransform),
) -> Result<Vec2, ViewportConversionError> {
    let (camera, camera_transform) = camera_query;
    camera.viewport_to_world_2d(camera_transform, pos)
}

pub fn get_throw_distance(
    window: Single<&Window>,
    press_pos: Res<PressPosition>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) -> Result<Vec2> {
    if window.cursor_position().is_none() {
        return Ok(Vec2::new(0., 0.));
    }

    let cursor_in_screen = window.cursor_position().unwrap();
    let cursor = get_world2d_coords(cursor_in_screen, camera_query.clone()).unwrap();

    let dist = normalize_throw_dir(cursor - press_pos.pos);

    let velocity_squared =
        GRAVITY * 2.0 * get_throw_speed(dist.length()) * dist.normalize_or_zero();

    return Ok(PICKAXE_MASS * velocity_squared.map(|c| c.abs().sqrt() * c.signum()));
}

#[test]
fn test_discrete() {
    let distance = THROW_MAX_LENGTH + 0.01;
    let discrete_dist = get_throw_discrete_distance(distance);
    assert!(distance >= discrete_dist);
    assert!(distance - 0.01 == discrete_dist);
}

#[test]
fn test_normalize() {
    // should only work with bigger distance
    let position = Vec2::new(10., THROW_MAX_LENGTH);
    let distance = get_throw_discrete_distance(position.length());
    let position_norm = normalize_throw_dir(position);
    assert!((position_norm.length() - distance).abs() < 0.001);

    let position = Vec2::new(3., 4.);
    let distance = get_throw_discrete_distance(position.length());
    let position_norm = normalize_throw_dir(position);
    assert!((position_norm.length() - distance).abs() >= 0.001);
}
