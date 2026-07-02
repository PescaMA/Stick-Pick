use avian2d::{collision::collider::CollidingEntities, dynamics::rigid_body::LinearVelocity};
use bevy::{math::vec3, prelude::*};

use crate::player::{Player, physics_bundles::PlayerPart};

const SHAKE_MULT: f32 = 0.1;
const SHAKE_STRENGTH: f32 = 0.1;
const SHAKE_TIME: f32 = 0.05;

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

        transl.translation = shake.initial_pos.translation
            + vec3(
                (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
                (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
                0.,
            );
    }
}

pub(crate) fn add_collision_shake(
    head_collisions: Query<(&CollidingEntities, &ChildOf), With<PlayerPart>>,
    parent: Query<&LinearVelocity, With<Player>>,
    transform: Query<&Transform>,
    shake: Query<&Shake>,
    camera: Query<Entity, With<Camera>>,
    mut commands: Commands,
) {
    for camera in camera {
        for (collision, child) in head_collisions {
            for _ in collision.iter() {
                if shake.get(camera).is_ok() {
                    continue;
                }
                if parent
                    .get(child.parent())
                    .map(|vel| vel.0.length() < 30.)
                    .unwrap_or(false)
                {
                    continue;
                }

                commands
                    .entity(camera)
                    .insert(Shake::default(transform.get(camera).unwrap().clone()));
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (shake, add_collision_shake));
}
