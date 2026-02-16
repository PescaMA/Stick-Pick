//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    game::player::{PlayerAssets, player},
    screens::Screen,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::{map::TilemapId, tiles::TilePos};
use bevy_map::{
    MapHandle, MapRoot, SpawnMapEvent,
    runtime::{MapLayerIndex, bevy_map_core},
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.add_systems(Update, name_tiles_on_spawn);
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
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
            children![
                player(400.0, &player_assets, &mut texture_atlas_layouts),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                ),
            ],
        ),
        MapHandle(asset_server.load("maps/licenta_jam.map.json")),
    ));
}

pub fn name_tiles_on_spawn(
    mut commands: Commands,
    added_tiles: Query<(Entity, &TilemapId), Added<TilePos>>,
    layer: Query<&MapLayerIndex>,
) {
    for (entity, childof) in added_tiles {
        let parent = childof.0;
        if layer.get(parent).is_ok() {
            let name = layer.get(parent).unwrap().0;
            commands.entity(entity).insert(Name::new(name.to_string()));
        }
    }
}
