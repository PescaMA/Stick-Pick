//! Player-specific behavior.

use avian2d::prelude::*;
use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};
use bevy_firefly::lights::PointLight2d;

use crate::{
    AppSystems, PausableSystems, SPRITE_SCALE,
    animation::PlayerAnimation,
    asset_tracking::LoadResource,
    movement::{MovementController, ScreenWrap},
};

const PLAYER_SPRITE_SIZE: f32 = 32.;
pub const PLAYER_SPAWN_POSITION: Vec3 = vec3(100., 30., 1.);

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (record_player_directional_input, apply_impulse_on_x)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(
    max_speed: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(
            player_assets.image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(SPRITE_SCALE).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        player_animation,
        PointLight2d {
            // for firefly lighting
            color: Color::srgb(0.2, 0.0, 1.0),
            range: PLAYER_SPRITE_SIZE * SPRITE_SCALE * 3.,
            intensity: 4.0,
            ..default()
        },
        (
            RigidBody::Dynamic, // affected by gravity/colissions
            // Capsule prevents catching on tile edges
            Collider::capsule(7.0, 8.0),
            CollisionLayers::from_bits(3, 3), // we are in layer 1 and collide with layer 1
            // LockedAxes::ROTATION_LOCKED,
            Friction::new(0.1),
            Restitution::new(0.3), // bounciness. 1 = perfectly elastic, 0 = no
            LinearVelocity::ZERO,  // start stationary
            CollidingEntities::default(), // track collisions
            GravityScale(1.0),
        ),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
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

fn apply_impulse_on_x(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyX) {
        return;
    }

    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x += 200.0;
        linear_velocity.x += 200.0;

        angular_velocity.0 += 0.5;
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    image: Handle<Image>,
    #[dependency]
    pub throw_sound: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            image: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            throw_sound: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
