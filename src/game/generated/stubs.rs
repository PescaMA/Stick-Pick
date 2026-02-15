use super::entities::*;
use bevy::prelude::*;
/// Called every frame for sticky entities
///
/// Use this to implement movement, AI, or other per-frame logic.
pub fn update_sticky(_time: Res<Time>, _query: Query<(Entity, &Transform, &Sticky), With<Sticky>>) {
}
/// Called when a sticky entity is spawned
///
/// Use this to set up additional components or initialize state.
pub fn on_sticky_spawned(
    mut _commands: Commands,
    _query: Query<(Entity, &Transform), Added<Sticky>>,
) {
}
/// Called when a sticky entity is removed
///
/// Use this for cleanup or spawn effects on death/removal.
pub fn on_sticky_removed(mut _commands: Commands, _removed: RemovedComponents<Sticky>) {}
/// Plugin that registers all generated stub systems
///
/// Add this to your app to enable the stub systems:
/// ```ignore
/// app.add_plugins(StubsPlugin);
/// ```
#[derive(Default)]
pub struct StubsPlugin;
impl Plugin for StubsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_sticky)
            .add_systems(Update, on_sticky_spawned)
            .add_systems(Update, on_sticky_removed);
    }
}
