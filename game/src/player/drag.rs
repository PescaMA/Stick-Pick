use bevy::{camera::ViewportConversionError, color::palettes::css::WHITE, prelude::*};

use crate::player::physics::PlayerPart;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(PressPosition::default());
    app.add_systems(Update, (draw_lines, add_player_observer, end_drag));
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
    println!("I am being pressed");
}

fn end_drag(evr_cursor: MessageReader<Pointer<DragEnd>>, mut press_pos: ResMut<PressPosition>) {
    if evr_cursor.len() > 0 {
        press_pos.currently_pressed = false;
    }
}

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

    info!("{} - > {}", cursor_in_screen, cursor);

    let dest = cursor;
    let dest_reflected = start * 2. - dest;

    gizmos.line_2d(start, dest, WHITE);
    gizmos.line_2d(start, dest_reflected, WHITE);

    println!("{}{}", press_pos.pos, dest);
}
