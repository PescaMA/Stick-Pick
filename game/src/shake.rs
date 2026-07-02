use avian2d::collision::collider::CollidingEntities;
use bevy::{math::vec3, prelude::*};

use crate::{level::ldtk_entities::Wall, player::physics_bundles::PlayerPart};

const SHAKE_MULT: f32 = 20.;
const SHAKE_STRENGTH: f32 = 5.;
const SHAKE_TIME: f32 = 0.1;

#[derive(Component)]
pub struct Shake {
    strength: f32,
    timer: Timer,
    initial_pos: Transform,
}

//decide how strong and how long will the shake be
impl Shake {
    pub fn default(initial_pos: Transform) -> Self {
        Self {
            strength: SHAKE_STRENGTH,
            timer: Timer::from_seconds(SHAKE_TIME, TimerMode::Once),
            initial_pos,
        }
    }
}

fn shake(
    mut shakable: Query<(Entity, &mut Shake, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut shake, mut transl) in shakable.iter_mut() {
        if shake.timer.tick(time.delta()).just_finished() {
            *transl = shake.initial_pos.clone();
            commands.entity(entity).remove::<Shake>();
            continue;
        }

        transl.translation += vec3(
            (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
            (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
            0.,
        );
    }
}

pub(crate) fn add_collision_shake(
    head_collisions: Query<&CollidingEntities, With<PlayerPart>>,
    transform: Query<&Transform>,
    camera: Query<Entity, With<Camera>>,
    mut commands: Commands,
) {
    for camera in camera {
        for collision in head_collisions {
            for col_entity in collision.iter() {
                commands
                    .entity(camera)
                    .insert(Shake::default(transform.get(*col_entity).unwrap().clone()));
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Update, (shake, add_collision_shake));
}
