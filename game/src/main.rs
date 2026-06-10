//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use avian2d::prelude::PhysicsPickingPlugin;
use bevy::{asset::AssetMetaCheck, ecs::system::NonSendMarker, prelude::*};

#[cfg(feature = "dev_native")]
use bevy_hotpatching_experiments::prelude::*;

use cli_template::*;
use winit::window::Icon;

mod camera;

pub mod level;
pub mod player;

const SPRITE_SOURCE_PX: f32 = 16.0;
const SPRITE_TARGET_PX: f32 = 32.0;
const SPRITE_SCALE: f32 = SPRITE_TARGET_PX / SPRITE_SOURCE_PX;

pub fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, camera::plugin));
}

fn set_window_icon(
    // we have to run from main thread
    _: NonSendMarker,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("game/assets/images/pickaxe.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    bevy::winit::WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
        if winit_windows.windows.is_empty() {
            return;
        }
        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });
}

fn main() -> AppExit {
    info!("starting app");
    let mut app = App::new();

    // Add Bevy plugins.
    app.add_plugins((
        #[cfg(feature = "dev_native")]
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
    app.add_plugins(plugin);
    app.add_systems(Startup, set_window_icon);
    app.run()
}
