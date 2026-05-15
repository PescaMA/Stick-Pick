use std::time::Duration;

use avian2d::{
    PhysicsPlugins,
    collision::collider::Collider,
    dynamics::{integrator::Gravity, rigid_body::RigidBody},
    schedule::PhysicsSchedule,
};
use bevy::{prelude::*, time::TimePlugin};
use bevy_ecs_ldtk::LdtkProjectHandle;

use crate::{
    level::ldtk_entities::spawn_collider,
    player::{Player, drag::PressPosition, movement::GRAVITY, physics::add_avian_body},
};

const PREDICT_TIME_S: f32 = 2.0;

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

    let collider_entities: Vec<(
        Option<RigidBody>,
        Option<Collider>,
        Option<Transform>,
        Option<Player>,
    )> = world
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

    let mut sim_world = world.get_resource_mut::<SimulationWorld>().unwrap();

    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugins::default()));
    app.insert_resource(Gravity(Vec2::new(0.0, GRAVITY)));
    app.finish();
    app.cleanup();

    sim_world.world = std::mem::replace(app.world_mut(), World::default());

    // sim_world
    //     .world
    //     .insert_resource(ColliderTreeDiagnostics::default());

    for (rig_opt, col_opt, tr_opt, player_opt) in collider_entities {
        let mut builder = sim_world.world.spawn_empty();
        if let Some(player) = player_opt {
            info!("WE HAVE PLAYERSSS");
            builder.insert(player);
            continue;
        }
        if let Some(rig) = rig_opt {
            builder.insert(rig);
        }
        if let Some(col) = col_opt {
            builder.insert(col);
        }
        if let Some(tr) = tr_opt {
            builder.insert(sanitize_transform(tr));
            info!("transform: {}, {}", tr.translation, tr.rotation);
        }
    }

    let add_player = sim_world.world.register_system(add_avian_body);
    // let _ = sim_world.world.run_system(add_player);

    info!("new world size: {}", sim_world.world.entity_count());

    sim_world.is_init = true;
}

fn sanitize_transform(mut t: Transform) -> Transform {
    if !t.translation.to_array().iter().all(|v| v.is_finite()) {
        t.translation = Vec3::ZERO;
    }
    let q = t.rotation;
    if ![q.x, q.y, q.z, q.w].iter().all(|v| v.is_finite()) {
        t.rotation = Quat::IDENTITY;
    }
    t
}

fn drag_physics_predict(
    window: Single<&Window>,
    press_pos: Res<PressPosition>,
    mut sim_world: ResMut<SimulationWorld>,
) {
    if !press_pos.currently_pressed || window.cursor_position().is_none() {
        return;
    }

    let sim_world = &mut sim_world.world;

    info!("in predict: ");

    if sim_world.get_resource_mut::<Time>().is_some() {
        // running in higher fps is laggy.
        let simulation_fps = 15.;

        let steps = (PREDICT_TIME_S * simulation_fps).ceil() as usize;
        for _ in 0..steps {
            // advance physics time by a substep
            sim_world
                .get_resource_mut::<Time>()
                .unwrap()
                .advance_by(Duration::from_secs_f64(1.0 / simulation_fps as f64));
            // run the physics schedule — replace `PhysicsSchedule` with the actual schedule name

            if sim_world.try_run_schedule(PhysicsSchedule).is_err() {
                info!("cannot find physics schedule");
            }
        }

        info!(
            "{}",
            sim_world
                .query_filtered::<&Transform, With<Player>>()
                .iter(sim_world)
                .next()
                .map(|x| x.translation)
                .unwrap_or_else(|| Vec3::splat(-999.))
        );
    }
}
