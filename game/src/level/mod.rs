//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    player::{PlayerAssets, player},
    screens::Screen,
};



// use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;


pub mod ldtk_entities;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ldtk_entities::plugin);
    app.load_resource::<LevelAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        (
            Name::new("Level"),
            DespawnOnExit(Screen::Gameplay),
            children![
                player(&player_assets, &mut texture_atlas_layouts),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                ),
            ],
            LdtkWorldBundle {
            ldtk_handle: asset_server.load("maps/rage_game.ldtk").into(),
            ..Default::default()},
        ),
    ));
}

