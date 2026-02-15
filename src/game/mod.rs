//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

use crate::game::generated::GeneratedPlugin;

mod animation;
mod camera;
mod generated;
pub mod level;
mod movement;
pub mod player;

const SPRITE_SOURCE_PX: f32 = 16.0;
const SPRITE_TARGET_PX: f32 = 32.0;
const SPRITE_SCALE: f32 = SPRITE_TARGET_PX / SPRITE_SOURCE_PX;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        camera::plugin,
        GeneratedPlugin,
    ));
}
