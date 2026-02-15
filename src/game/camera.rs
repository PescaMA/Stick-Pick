use crate::game::player::Player;
use bevy::prelude::*;

use bevy_firefly::prelude::*;

const CAMERA_DECAY_RATE: f32 = 2.;

pub(super) fn plugin(app: &mut App) {
	 app.add_plugins(FireflyPlugin,);
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d, FireflyConfig::default()));
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // smooth effect to camera movement that follows player

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
