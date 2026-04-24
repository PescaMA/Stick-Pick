use crate::{
    SPRITE_SCALE,
    level::LevelBoundaries,
    player::{PLAYER_SPAWN_POSITION, Player},
};
use bevy::{prelude::*, window::PrimaryWindow};

use bevy_firefly::prelude::*;

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;
const CAMERA_DECAY_RATE: f32 = 2.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FireflyPlugin);
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, (update_camera, resize_system));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0 / SPRITE_SCALE,
            ..OrthographicProjection::default_2d()
        }),
        Camera2d,
        FireflyConfig::default(),
    ));
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    level_boundary: Res<LevelBoundaries>,
) {
    let y = player.translation.y.max(PLAYER_SPAWN_POSITION.y); // let camera go only up

    let direction = Vec3::new(level_boundary.center_x(), y, camera.translation.z);

    // smooth effect to camera movement that follows player

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn resize_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    let window = windows.single().unwrap();
    let win_w = window.width();
    let win_h = window.height();

    if win_h == 0. {
        return; // window minimized
    }

    let scale_x = win_w / VIRTUAL_WIDTH;
    let scale_y = win_h / VIRTUAL_HEIGHT;
    let scale = scale_x.min(scale_y);

    for mut transform in q_camera.iter_mut() {
        transform.scale = Vec3::splat(1.0 / scale);
    }
}
