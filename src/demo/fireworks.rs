use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_enoki::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnokiPlugin);
    // app.add_systems(OnExit(Screen::Launchpad), despawn_resources);
    app.add_systems(
        Update,
        (
            (setup.run_if(in_state(Screen::Launchpad).and(input_just_pressed(KeyCode::Space))))
                .chain(),
        ),
    );
}

fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
    server: Res<AssetServer>,
) {
    // // minimal setup
    // // white quads with a default effect
    // cmd.spawn(
    //     // the main component.
    //     // holds a material handle.
    //     // defaults to a simple white color quad.
    //     // has required components
    //     ParticleSpawner::default(),
    // );

    // bring in your own effect asset from a ron file
    // (hot reload by default)
    cmd.spawn((
        ParticleSpawner::default(),
        OneShot::Despawn,
        // the effect components holds the baseline
        // effect asset.
        ParticleEffectHandle(server.load("shaders/test_firework.particle.ron")),
    ));

    // // now with a sprite sheet animation over lifetime
    // let sprite_material = materials.add(
    //     // the other args (hframes and vframes) defines how the sprite sheet is divided for animating,
    //     // you can also just use `form_texture` for a single sprite
    //     SpriteParticle2dMaterial::new(server.load("shaders/particle.png"), 6, 1),
    // );

    // cmd.spawn((
    //     ParticleSpawner(sprite_material),
    //     ParticleEffectHandle(server.load("shaders/test_firework.particle.ron")),
    // ));
}

pub(crate) fn remove_finished_spawner(
    mut cmd: Commands,
    spawner: Query<(Entity, &ParticleStore, &ParticleSpawnerState, &OneShot)>,
) {
    spawner
        .iter()
        .for_each(|(entity, store, controller, one_shot)| {
            if matches!(one_shot, OneShot::Despawn) && !controller.active && store.len() == 0 {
                cmd.entity(entity).despawn();
            }
        })
}
