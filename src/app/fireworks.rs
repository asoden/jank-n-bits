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

    cmd.spawn(spawn_shrimp(position, materials, server));
}

fn spawn_shrimp(
    position: Vec3,
    materials: &mut ResMut<Assets<SpriteParticle2dMaterial>>,
    server: &Res<AssetServer>,
) -> impl Bundle {
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

    let particle_effect = Particle2dEffect {
        spawn_rate: 0.1,
        spawn_amount: 500,
        emission_shape: EmissionShape::Circle(0.0),
        lifetime: Rval(3.0, 0.5),
        linear_speed: Some(Rval(1000.0, 0.5)),
        linear_acceleration: Some(Rval(0.0, 0.0)),
        direction: Some(Rval(Vec2::new(0.0, 1.0), 0.3)),
        angular_speed: Some(Rval(10.0, 0.1)),
        angular_acceleration: Some(Rval(0.0, 0.0)),
        scale: Some(Rval(10000.0, 10000.0)),
        color: None,
        gravity_direction: Some(Rval(Vec2::new(0.0, -1.0), 0.0)),
        gravity_speed: Some(Rval(500.0, 1.0)),
        linear_damp: Some(Rval(30.0, 0.8)),
        angular_damp: Some(Rval(1.0, 0.0)),
        scale_curve: None,
        // scale_curve: Some(MultiCurve {
        //     points: vec![
        //         (10.0, 0.0, None),
        //         (10.0, 0.15, Some(EaseFunction::BounceOut)),
        //         (1.0, 0.15, Some(EaseFunction::SineOut)),
        //     ],
        // }),
        color_curve: Some(MultiCurve {
            points: vec![
                (
                    LinearRgba {
                        red: 0.9484073,
                        green: 0.37821338,
                        blue: 0.37821338,
                        alpha: 1.0,
                    },
                    0.0,
                    None,
                ),
                (
                    LinearRgba {
                        red: 0.21493901,
                        green: 0.47699985,
                        blue: 0.17750177,
                        alpha: 1.0,
                    },
                    0.2,
                    None,
                ),
                (
                    LinearRgba {
                        red: 0.006995418,
                        green: 0.21586034,
                        blue: 0.5149177,
                        alpha: 1.0,
                    },
                    0.3,
                    None,
                ),
                (
                    LinearRgba {
                        red: 0.2,
                        green: 0.2,
                        blue: 0.2,
                        alpha: 1.0,
                    },
                    0.4,
                    None,
                ),
                (
                    LinearRgba {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                        alpha: 1.0,
                    },
                    1.0,
                    None,
                ),
            ],
        }),
    };

    (
        ParticleSpawner(sprite_material),
        OneShot::Despawn,
        ParticleEffectInstance(Some(particle_effect)),
        // ParticleEffectHandle(server.load("shaders/test_firework.particle.ron")),
        Transform::from_translation(position),
    )
}
