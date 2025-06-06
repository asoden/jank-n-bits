use bevy::prelude::*;
use bevy_enoki::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnokiPlugin);
    app.add_systems(
        Update,
        (
            (setup,).chain(),
            // .run_if(resource_exists::<PlayerAssets>)
            // .in_set(AppSystems::Update),)
            // .in_set(PausableSystems),
        ),
    );
}

fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
    server: Res<AssetServer>,
) {
    // minimal setup
    // white quads with a default effect
    cmd.spawn(
        // the main component.
        // holds a material handle.
        // defaults to a simple white color quad.
        // has required components
        ParticleSpawner::default(),
    );

    // bring in your own effect asset from a ron file
    // (hot reload by default)
    cmd.spawn((
        ParticleSpawner::default(),
        // the effect components holds the baseline
        // effect asset.
        ParticleEffectHandle(server.load("shaders/firework.particle.ron")),
    ));

    // now with a sprite sheet animation over lifetime
    let sprite_material = materials.add(
        // the other args (hframes and vframes) defines how the sprite sheet is divided for animating,
        // you can also just use `form_texture` for a single sprite
        SpriteParticle2dMaterial::new(server.load("shaders/particle.png"), 6, 1),
    );

    cmd.spawn((
        ParticleSpawner(sprite_material),
        ParticleEffectHandle(server.load("shaders/firework.particle.ron")),
    ));
}
