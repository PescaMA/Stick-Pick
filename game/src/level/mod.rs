//! Spawn the main level.

use crate::{asset_tracking::LoadResource, audio::music, player::Player, screens::Screen};

// use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_ecs_ldtk::{
    LdtkProjectHandle, LdtkWorldBundle, LevelEvent,
    assets::{LdtkProject, LevelMetadataAccessor},
};

pub mod ldtk_entities;

#[derive(Resource, Default, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelBoundaries {
    pub(crate) origin: Vec2,
    pub(crate) length: Vec2,
}

impl LevelBoundaries {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> LevelBoundaries {
        Self {
            origin: Vec2::new(x as f32, y as f32),
            length: Vec2::new(w as f32, h as f32),
        }
    }

    pub fn center_x(&self) -> f32 {
        return self.origin.x + self.length.x / 2.;
    }
    // pub fn center_y(&self) -> f32 {
    //     return self.origin.y + self.length.y / 2.;
    // }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(LevelBoundaries::default());
    app.add_plugins(ldtk_entities::plugin);
    app.load_resource::<LevelAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_systems(Update, print_lvl_bounds);
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
            music: assets.load("audio/music/game_seth.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(((
        Name::new("Level"),
        DespawnOnExit(Screen::Gameplay),
        children![(
            Name::new("Gameplay Music"),
            music(level_assets.music.clone())
        ),],
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("maps/rage_game.ldtk").into(),
            ..Default::default()
        },
    ),));
}

pub fn print_lvl_bounds(
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut level_boundary: ResMut<LevelBoundaries>,
    player_pos: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            // note: level.world_x, level.world_y are not useful, the map is loaded at 0,0
            *level_boundary = LevelBoundaries::new(0, 0, level.px_wid, level.px_hei);

            for mut camera in camera.iter_mut() {
                camera.translation = Vec3 {
                    x: level_boundary.center_x(),
                    y: player_pos.iter().next().unwrap().translation.y,
                    z: camera.translation.z.clone(),
                }
            }
        }
    }
    Ok(())
}
