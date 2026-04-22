use bevy_ecs_ldtk::{app::LdtkEntityAppExt, *};
use bevy::prelude::*;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(LevelSelection::index(0))
    .add_plugins(LdtkPlugin)
.register_ldtk_entity::<PlayerBundle>("Player")
    .register_ldtk_entity::<GoalBundle>("Goal");}