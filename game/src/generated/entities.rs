use bevy::prelude::*;
use bevy_map::prelude::*;
#[derive(Component, Debug, Clone, Default, MapEntity)]
#[map_entity(type_name = "sticky")]
pub struct Sticky {}
