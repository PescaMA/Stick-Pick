use std::time::Duration;

use avian2d::{
    PhysicsPlugins,
    collision::collider::Collider,
    dynamics::{
        integrator::Gravity,
        rigid_body::{
            AngularVelocity, LinearVelocity, RigidBody,
            forces::{Forces, ReadRigidBodyForces, WriteRigidBodyForces},
        },
    },
    physics_transform::{Position, position_to_transform},
    schedule::{Physics, PhysicsSchedule},
};
use bevy::{
    ecs::system::RunSystemOnce, prelude::*, scene::ScenePlugin, state::app::StatesPlugin,
    time::TimePlugin,
};
use bevy_ecs_ldtk::LdtkProjectHandle;
use cli_template::{PausableSystems, Pause};

use crate::{
    level::{LevelBoundaries, ldtk_entities::spawn_collider},
    player::{
        Player,
        drag::PressPosition,
        drag_helper::get_throw_distance,
        movement::{self, GRAVITY},
        physics::add_avian_body,
    },
};

const PREDICT_TIME_S: f32 = 3.0;
// running in higher fps is laggy.
const SIMULATION_FPS: f32 = 20.;

#[derive(Resource, Default)]
struct SimulationWorld {
    world: World,
    is_init: bool,
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(SimulationWorld::default());
    app.add_systems(Update, (init_world).after(spawn_collider));
    app.add_systems(Update, (drag_physics_predict,));
}

fn init_world(world: &mut World) {
    if world
        .query_filtered::<(), With<LdtkProjectHandle>>()
        .iter(world)
        .next()
        .is_none()
        || world
            .get_resource_mut::<SimulationWorld>()
            .map(|r| r.is_init)
            .unwrap_or_else(|| true)
    {
        return;
    }

    if world.get_resource_mut::<SimulationWorld>().is_none() {
        return;
    };

    let collider_entities: Vec<_> = world
        .query::<(
            Option<&RigidBody>,
            Option<&Collider>,
            Option<&Transform>,
            Option<&Player>,
        )>()
        .iter(world)
        .map(|(r, c, t, p)| (r.cloned(), c.cloned(), t.cloned(), p.cloned()))
        .collect();

    if collider_entities.is_empty()
        || !collider_entities
            .iter()
            .any(|(_, _, _, player_opt)| player_opt.is_some())
    {
        return;
    }

    let boundaries = world.get_resource::<LevelBoundaries>().unwrap().clone();
    let mut sim_world = world.get_resource_mut::<SimulationWorld>().unwrap();

    let mut app = App::new();

    app.add_plugins((
        StatesPlugin,
        TimePlugin,
        AssetPlugin::default(), // for scene
        ScenePlugin,            // for update
        PhysicsPlugins::default(),
    ));

    // Set up the `Pause` state.
    // app.add_systems(Update, log_transitions::<Screen>);
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    app.add_plugins(movement::plugin);
    app.insert_resource(Gravity(Vec2::new(0.0, GRAVITY)));
    app.insert_resource(PressPosition::default());
    app.insert_resource(boundaries);

    app.finish();
    app.cleanup();

    sim_world.world = std::mem::replace(app.world_mut(), World::default());

    for (rig_opt, col_opt, tr_opt, player_opt) in collider_entities {
        let mut builder = sim_world.world.spawn_empty();
        if let Some(player) = player_opt {
            builder.insert(player);
            continue; // the player will be inserted later
        }
        if let Some(rig) = rig_opt {
            builder.insert(rig);
        }
        if let Some(col) = col_opt {
            builder.insert(col);
        }
        if let Some(tr) = tr_opt {
            builder.insert(tr);
        }
    }

    let add_player = sim_world.world.register_system(add_avian_body);
    let _ = sim_world.world.run_system(add_player);

    // sim_world.world.resource_mut::<Time<Physics>>().pause();

    info!("new world size: {}", sim_world.world.entity_count());

    sim_world.is_init = true;
}

fn drag_physics_predict(
    window: Single<&Window>,
    press_pos: Res<PressPosition>,
    camera_query: Single<(&Camera, &GlobalTransform)>,

    mut sim_world: ResMut<SimulationWorld>,
    player_pos: Query<(&Transform, &LinearVelocity, &AngularVelocity, &Position), With<Player>>,
) {
    if !press_pos.currently_pressed || window.cursor_position().is_none() {
        return;
    }

    let sim_world = &mut sim_world.world;

    // move simulated player to current player position
    let player_pos = player_pos.iter().next().unwrap();

    let (mut tr, mut lv, mut av, mut pos) = sim_world
        .query_filtered::<(
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Position,
        ), With<Player>>()
        .iter_mut(sim_world)
        .next()
        .unwrap();
    tr.translation = player_pos.0.translation;
    *lv = player_pos.1.clone();
    *av = player_pos.2.clone();
    *pos = player_pos.3.clone();

    *sim_world.get_resource_mut::<PressPosition>().unwrap() = press_pos.clone();

    if sim_world.get_resource_mut::<Time>().is_some() {
        let impulse_pos = press_pos.pos;
        let impulse_velocity = get_throw_distance(window, press_pos, camera_query).unwrap();

        let steps = (PREDICT_TIME_S * SIMULATION_FPS).ceil() as usize;

        let mut velocity: Vec<Vec2> = Vec::new();

        for mut forces in sim_world.query::<Forces>().iter_mut(sim_world) {
            forces.apply_linear_impulse_at_point(impulse_velocity, impulse_pos);
        }

        info!(
            "FORCES : {:?}, {:?}",
            sim_world
                .query::<Forces>()
                .iter(sim_world)
                .next()
                .unwrap()
                .position()
                .clone(),
            sim_world
                .query::<Forces>()
                .iter(sim_world)
                .next()
                .unwrap()
                .accumulated_linear_acceleration()
        );

        // HOW DO I CORRECTLY RUN THE WORLDDDD?????????????????????
        for _ in 0..steps {
            // advance physics time by a substep
            sim_world
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f32(1.0 / SIMULATION_FPS));

            sim_world
                .resource_mut::<Time<Physics>>()
                .advance_by(Duration::from_secs_f32(1.0 / SIMULATION_FPS));

            if sim_world.try_run_schedule(PhysicsSchedule).is_err() {
                info!("cannot find physics schedule");
            }
            if sim_world.try_run_schedule(Update).is_err() {
                info!("cannot find update schedule");
            }
            if sim_world.try_run_schedule(RunFixedMainLoop).is_err() {
                info!("cannot find fixed main schedule");
            }
            let _ = sim_world.run_system_once(position_to_transform);

            velocity.push(
                sim_world
                    .query_filtered::<&LinearVelocity, With<Player>>()
                    .iter(sim_world)
                    .next()
                    .map(|x| x.0)
                    .unwrap_or_else(|| Vec2::splat(-999.)),
            );
        }

        info!(
            "FORCES  END: {:?}, {:?}",
            sim_world
                .query::<Forces>()
                .iter(sim_world)
                .next()
                .unwrap()
                .position()
                .clone(),
            sim_world
                .query::<Forces>()
                .iter(sim_world)
                .next()
                .unwrap()
                .accumulated_linear_acceleration()
        );

        // info!(
        //     "Ending player position at {:?}: {:?} \n {}",
        //     sim_world.resource_mut::<Time<Physics>>().elapsed(),
        //     velocity,
        //     sim_world
        //         .query_filtered::<&Transform, With<Player>>()
        //         .iter(sim_world)
        //         .next()
        //         .map(|x| x.translation)
        //         .unwrap_or_else(|| Vec3::splat(-999.))
        // );
    }
}
