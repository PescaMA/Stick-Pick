//! Auto-generated code from bevy_map_editor
//!
//! This module is regenerated when you save your map project with code generation enabled.
//! Do not edit manually - your changes will be overwritten!

use engine::prelude::*;

mod entities;
mod stubs;

use bevy_map::MapEntityExt;
pub use entities::*;
pub use stubs::StubsPlugin;

/// Plugin that registers all generated systems and components
pub struct GeneratedPlugin;

impl Plugin for GeneratedPlugin {
    fn build(&self, app: &mut App) {
        app.register_map_entity::<Sticky>();

        // Add plugins
        app.add_plugins(StubsPlugin);
    }
}
