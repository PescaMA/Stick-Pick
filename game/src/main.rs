//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use cli_template::{screens::Screen, *};
use engine::prelude::*;

use generated::GeneratedPlugin;

use crate::level::spawn_level;

mod animation;
mod camera;
mod generated;
pub mod level;
mod movement;
pub mod player;

const SPRITE_SOURCE_PX: f32 = 16.0;
const SPRITE_TARGET_PX: f32 = 32.0;
const SPRITE_SCALE: f32 = SPRITE_TARGET_PX / SPRITE_SOURCE_PX;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        camera::plugin,
        GeneratedPlugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
}

fn main() -> AppExit {

	info!("starting app");
    let mut app = App::new();
    
    // Add Bevy plugins.
    app.add_plugins((
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
        MapRuntimePlugin,
        MapCollisionPlugin, // also adds PhysicsPlugins::default());
    ));
    
    
    app.add_plugins(AppPlugin);
    app.add_plugins(plugin).run()
}
