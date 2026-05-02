//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use avian2d::prelude::PhysicsPickingPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_hotpatching_experiments::prelude::*;
use cli_template::*;

mod camera;

pub mod level;
pub mod player;

const SPRITE_SOURCE_PX: f32 = 16.0;
const SPRITE_TARGET_PX: f32 = 32.0;
const SPRITE_SCALE: f32 = SPRITE_TARGET_PX / SPRITE_SOURCE_PX;

pub fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, camera::plugin));
}

fn main() -> AppExit {
    info!("starting app");
    let mut app = App::new();

    // Add Bevy plugins.
    app.add_plugins((
        SimpleSubsecondPlugin::default(), // for hotpatching
        PhysicsPickingPlugin,
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "My Bevy App".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    ));

    app.add_plugins(AppPlugin);
    app.add_plugins(plugin).run()
}
