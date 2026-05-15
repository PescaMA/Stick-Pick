use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_firefly::lights::PointLight2d;

use crate::{
    SPRITE_SCALE,
    player::{
        PLAYER_SPRITE_SIZE,
        movement::{IgnoreSticky, MovementController, ScreenBlock},
    },
};

pub const HEAD_MASS: f32 = 200.;
pub const HANDLE_MASS: f32 = 100.;
pub const PICKAXE_MASS: f32 = HEAD_MASS + HANDLE_MASS;

const DAMPING_FACTOR: f32 = 0.5;
const MAX_ANGULAR_SPEED: f32 = 12.;
const LIGHT_COLOR: Color = Color::srgb(0.556, 0.654, 0.238);
const LIGHT_INTENSITY: f32 = 3.2;
const LIGHT_RANGE: f32 = PLAYER_SPRITE_SIZE * SPRITE_SCALE * 3.;

const PICKHEAD_SIZE: Vec2 = Vec2::new(3., 13.);
const PICKHANDLE_SIZE: Vec2 = Vec2::new(2., 18.);
const DRAG_PADDING: f32 = 5.;

#[derive(Component, Default, Clone, PartialEq)]
pub enum PlayerPart {
    #[default]
    PickHead,
    PickHandle,
}

#[derive(Component)]
pub struct PlayerPartHitbox;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub gravity_scale: GravityScale,
    pub movement_controller: MovementController,
    pub rigid_body: RigidBody,
    pub sleeping_disabled: SleepingDisabled,
    pub angular_damping: AngularDamping,
    pub linear_damping: LinearDamping,
    pub max_angular_speed: MaxAngularSpeed,
    pub screen_block: ScreenBlock,
    pub ignore_sticky: IgnoreSticky,
    pub point_light: PointLight2d,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            gravity_scale: GravityScale(1.0),
            movement_controller: MovementController { ..default() },
            rigid_body: RigidBody::Dynamic,
            sleeping_disabled: SleepingDisabled,
            angular_damping: AngularDamping(DAMPING_FACTOR),
            linear_damping: LinearDamping(DAMPING_FACTOR),
            max_angular_speed: MaxAngularSpeed(MAX_ANGULAR_SPEED),
            screen_block: ScreenBlock,
            ignore_sticky: IgnoreSticky::default(),
            point_light: PointLight2d {
                color: LIGHT_COLOR,
                range: LIGHT_RANGE,
                intensity: LIGHT_INTENSITY,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct CommonPlayerPartBundle {
    pub linear_velocity: LinearVelocity,
    pub colliding_entities: CollidingEntities,
    pub debug_render: DebugRender,
    pub collision_layers: CollisionLayers,
    pub friction: Friction,
}
impl Default for CommonPlayerPartBundle {
    fn default() -> Self {
        Self {
            linear_velocity: LinearVelocity::ZERO, // start stationary
            colliding_entities: CollidingEntities::default(), // track collisions
            debug_render: DebugRender::default()
                .with_collider_color(Color::linear_rgba(0.6, 0.4, 0.4, 0.5))
                .with_aabb_color(Color::WHITE.with_alpha(0.0)),
            collision_layers: CollisionLayers::from_bits(2, 1), // we are in layer 2 and collide with layer 1
            friction: Friction::new(0.3),                       // friction with other colliders
        }
    }
}

#[derive(Bundle)]
pub struct PickHeadBundle {
    pub name: Name,
    pub player_part: PlayerPart,
    pub transform: Transform,
    pub mass: Mass,
    pub collider: Collider,
    pub restitution: Restitution,
    pub common: CommonPlayerPartBundle,
}

impl Default for PickHeadBundle {
    fn default() -> Self {
        Self {
            name: Name::new("PickHead"),
            player_part: PlayerPart::PickHead,
            transform: Transform::from_xyz(0.0, 10.0, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            mass: Mass(HEAD_MASS),
            collider: Collider::capsule(PICKHEAD_SIZE.x, PICKHEAD_SIZE.y),
            restitution: Restitution::new(0.5),
            common: CommonPlayerPartBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct HandleBundle {
    pub name: Name,
    pub player_part: PlayerPart,
    pub transform: Transform,
    pub mass: Mass,
    pub collider: Collider,
    pub restitution: Restitution,
    pub common: CommonPlayerPartBundle,
}

impl Default for HandleBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Handle"),
            player_part: PlayerPart::PickHandle,
            transform: Transform::from_xyz(0.0, -4.0, 0.0),
            mass: Mass(HANDLE_MASS),
            collider: Collider::capsule(PICKHANDLE_SIZE.x, PICKHANDLE_SIZE.y),
            restitution: Restitution::new(0.3),
            common: CommonPlayerPartBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct DragHitboxBundle {
    pub hitbox: PlayerPartHitbox,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub sensor: Sensor,
    pub debug_render: DebugRender,
}

impl DragHitboxBundle {
    fn common() -> Self {
        Self {
            hitbox: PlayerPartHitbox,
            sensor: Sensor,

            collision_layers: CollisionLayers::from_bits(2, 0),
            collider: Collider::default(), // placeholder; replaced by specific constructor below
            debug_render: DebugRender::default(),
        }
    }

    pub fn for_handle() -> Self {
        let mut b = Self::common();
        b.collider = Collider::capsule(PICKHANDLE_SIZE.x + DRAG_PADDING, PICKHANDLE_SIZE.y);
        b
    }

    pub fn for_head() -> Self {
        let mut b = Self::common();
        b.collider = Collider::capsule(PICKHEAD_SIZE.x + DRAG_PADDING, PICKHEAD_SIZE.y);
        b
    }
}
