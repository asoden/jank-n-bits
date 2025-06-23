use bevy::prelude::*;
use bevy_enoki::prelude::*;

use crate::app::launcher::ProjectileExplosionEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnokiPlugin);
    app.add_systems(Update, handle_explosions);
}

fn handle_explosions(
    mut commands: Commands,
    mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
    mut explosion_events: EventReader<ProjectileExplosionEvent>,
    server: Res<AssetServer>,
) {
    for event in explosion_events.read() {
        spawn_firework(&mut commands, &mut materials, event.position, &server);
    }
}

fn spawn_firework(
    cmd: &mut Commands,
    materials: &mut ResMut<Assets<SpriteParticle2dMaterial>>,
    position: Vec3,
    server: &Res<AssetServer>,
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
    // cmd.spawn((
    //     ParticleSpawner::default(),
    //     OneShot::Despawn,
    //     // the effect components holds the baseline
    //     // effect asset.
    //     ParticleEffectHandle(server.load("shaders/test_firework.particle.ron")),
    //     Transform::from_translation(position),
    // ));

    // now with a sprite sheet animation over lifetime
    let sprite_material = materials.add(
        // the other args (hframes and vframes) defines how the sprite sheet is divided for animating,
        // you can also just use `form_texture` for a single sprite
        SpriteParticle2dMaterial::new(
            server.load("images/FreePixelFood/Sprite/Food/Shrimp.png"),
            1,
            1,
        ),
    );

    cmd.spawn((
        ParticleSpawner(sprite_material),
        OneShot::Despawn,
        ParticleEffectHandle(server.load("shaders/test_firework.particle.ron")),
        Transform::from_translation(position),
    ));
}
