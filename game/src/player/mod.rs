//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

// pub mod drag_simulation;
pub mod movement;
pub mod physics;
pub mod physics_bundles;
pub mod sounds;

use rand::seq::IndexedRandom;

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, player::movement::MovementController,
};

const PLAYER_SPRITE_SIZE: f32 = 32.;
pub const PLAYER_SPAWN_POSITION: Vec3 = vec3(100., 30., 1.);

#[derive(Resource, Default)]
struct Flight {
    pub enabled: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.insert_resource(Flight::default());

    app.add_plugins((physics::plugin, movement::plugin, sounds::plugin));

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (record_player_directional_input, toggle_flight)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

fn toggle_flight(mut flight: ResMut<Flight>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyF) {
        flight.enabled = !flight.enabled;
    }
}

fn record_player_directional_input(
    flight: Res<Flight>,
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    if !flight.enabled {
        return;
    }
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    image: Handle<Image>,
    #[dependency]
    pub throw_sound: Vec<Handle<AudioSource>>,
    pub hit_sticky_sound: Vec<Handle<AudioSource>>,
    pub hit_non_sticky_sound: Vec<Handle<AudioSource>>,
}

impl PlayerAssets {
    fn get_random(source: Vec<Handle<AudioSource>>) -> Option<Handle<AudioSource>> {
        let rng = &mut rand::rng();
        source.choose(rng).cloned()
    }
    pub fn get_random_throw_sound(&self) -> Handle<AudioSource> {
        Self::get_random(self.throw_sound.clone()).unwrap()
    }
    pub fn get_random_sticky_sound(&self) -> Handle<AudioSource> {
        Self::get_random(self.hit_sticky_sound.clone()).unwrap()
    }
    pub fn get_random_non_sticky_sound(&self) -> Handle<AudioSource> {
        Self::get_random(self.hit_non_sticky_sound.clone()).unwrap()
    }
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            image: assets.load_with_settings(
                "images/pickaxe.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            throw_sound: vec![assets.load("audio/sound_effects/throw_sypherzent.wav")],
            hit_sticky_sound: vec![assets.load("audio/sound_effects/pickaxe_igroglaz.mp3")],
            hit_non_sticky_sound: vec![assets.load("audio/sound_effects/pickaxe_noisyredfox.ogg")],
        }
    }
}
