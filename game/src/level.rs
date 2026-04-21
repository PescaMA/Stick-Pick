//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    player::{PlayerAssets, player},
    screens::Screen,
};
use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_map::{MapHandle, runtime::MapLayerIndex};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.add_systems(Update, (name_tiles_on_spawn, name_collisions_on_spawn));
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
                player(&player_assets, &mut texture_atlas_layouts),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                ),
            ],
        ),
        MapHandle(asset_server.load("maps/licenta_jam.map.json")),
    ));
}

// it took 2 hrs to write ...
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

fn name_collisions_on_spawn(
    mut commands: Commands,
    query: Query<(Entity, &CollisionLayers), Added<CollisionLayers>>,
) {
    for (ent, layers) in query.iter() {
        // get the membership bitmask
        let membership_bits = layers.memberships;
        if membership_bits != 0 {
            // find lowest set bit = layer index used when spawning
            let layer_index = membership_bits.trailing_zeros(); // u32
            commands
                .entity(ent)
                .insert(Name::new(layer_index.to_string()));

            // info!(
            //     "Entity {:?} membership bits {:b}, layer index {}",
            //     ent, membership_bits.0, layer_index
            // );
        } else {
            // info!("Entity {:?} has empty membership", ent);
        }
    }
}

// pub fn name_collisions_on_spawn(
//     mut commands: Commands,
//     added_tiles: Query<Entity, Added<MapCollider>>,
//     layer: Query<&MapLayerIndex>,
// ) {
//     for entity in added_tiles {
//         if layer.get(entity).is_ok() {
//             let name = layer.get(entity).unwrap().0;
//             commands.entity(entity).insert(Name::new(name.to_string()));
//         }
//     }
// }
