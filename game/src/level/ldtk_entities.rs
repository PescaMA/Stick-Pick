use avian2d::{
    collision::collider::CollisionLayers,
    prelude::{Collider, RigidBody},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    *,
};
use bevy_firefly::occluders::Occluder2d;

use crate::{SPRITE_SOURCE_PX, player::Player};

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    player: Player,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Component)]
pub struct Sticky;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Default, Bundle, LdtkIntCell)]
struct StickyBundle {
    wall: Wall,
    sticky: Sticky,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(LevelSelection::index(0))
        .add_plugins(LdtkPlugin)
        .register_default_ldtk_int_cell_for_layer::<StickyBundle>("Collisions_Sticky")
        .register_default_ldtk_int_cell_for_layer::<WallBundle>("Collisions_Non_Sticky")
        //.register_ldtk_int_cell::<WallBundle>(2)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal");

    app.add_systems(Update, spawn_collider);
}

pub(crate) fn spawn_collider(walls: Query<Entity, Added<Wall>>, mut commands: Commands) {
    for wall in walls.iter() {
        commands.entity(wall).insert((
            Occluder2d::rectangle(SPRITE_SOURCE_PX, SPRITE_SOURCE_PX).with_opacity(0.5),
            Collider::rectangle(SPRITE_SOURCE_PX, SPRITE_SOURCE_PX),
            RigidBody::Static,
            CollisionLayers::from_bits(1, 2),
        ));
    }
}
