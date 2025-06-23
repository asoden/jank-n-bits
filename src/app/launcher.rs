use bevy::prelude::*;

use crate::{
    app::{score::ScoreEvent, uap::{DestroyUapEvent, Uap}},
    asset_tracking::LoadResource,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ProjectileExplosionEvent>();

    app.register_type::<Launcher>();
    app.register_type::<LauncherCrank>();

    app.register_type::<LauncherAssets>();
    app.register_type::<LauncherCrankAssets>();

    app.load_resource::<LauncherAssets>();
    app.load_resource::<LauncherCrankAssets>();

    app.add_systems(OnExit(Screen::Launchpad), despawn_launcher);
    app.add_systems(
        Update,
        ((
            launcher_rotation,
            launcher_shooting,
            launcher_crank_rotation,
            projectile_movement,
            projectile_collision,
            cleanup_projectiles,
        )
            .chain()
            .run_if(in_state(Screen::Launchpad)),),
    );
}

#[derive(Event)]
pub struct ProjectileExplosionEvent {
    pub position: Vec3,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Launcher {
    rotation_speed: i32,
    projectile_speed: i32,
    height: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct LauncherCrank {
    rotation_speed: i32,
}

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    distance: f32,
    damage: f32,
}

pub fn launcher(
    launcher_assets: &LauncherAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let launcher_height = 40;

    let layout = TextureAtlasLayout::from_grid(UVec2::new(12, 48), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Sprite {
            image: launcher_assets.launcher.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            }),
            anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.35)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -450.0, 0.0)),
        Launcher {
            rotation_speed: 2,
            projectile_speed: 500,
            height: launcher_height,
        },
    )
}

pub fn launcher_crank(
    launcher_crank_assets: &LauncherCrankAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Sprite {
            image: launcher_crank_assets.launcher_crank.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -450.0, 1.0)),
        LauncherCrank { rotation_speed: 6 },
    )
}

fn launcher_rotation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Launcher)>,
) {
    for (mut transform, launcher) in query.iter_mut() {
        let mut rotation_direction = 0.0;

        // Check for left rotation
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_direction += 1.0;
        }

        // Check for right rotation
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_direction -= 1.0;
        }

        // Apply rotation
        if rotation_direction != 0.0 {
            let rotation_amount =
                rotation_direction * launcher.rotation_speed as f32 * time.delta_secs();
            transform.rotate_z(rotation_amount);

            // Clamp rotation to reasonable bounds
            let current_rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
            if current_rotation > 1.2 {
                transform.rotation = Quat::from_rotation_z(1.2);
            } else if current_rotation < -1.2 {
                transform.rotation = Quat::from_rotation_z(-1.2);
            }
        }
    }
}

fn launcher_crank_rotation(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &LauncherCrank)>,
) {
    for (mut transform, launcher_crank) in query.iter_mut() {
        let mut rotation_direction = 0.0;

        // Check for right rotation; rotate opposite of the launcher
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_direction -= 1.0;
        }

        // Check for left rotation; rotate opposite of the launcher
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_direction += 1.0;
        }

        // Apply rotation
        if rotation_direction != 0.0 {
            let rotation_amount =
                rotation_direction * launcher_crank.rotation_speed as f32 * time.delta_secs();
            transform.rotate_z(rotation_amount);

            // Ideally only rotate the crank while the launcher is rotating
            let current_rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
            if current_rotation > 3.6 {
                transform.rotation = Quat::from_rotation_z(3.6);
            } else if current_rotation < -3.6 {
                transform.rotation = Quat::from_rotation_z(-3.6);
            }
        }
    }
}

fn launcher_shooting(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    launcher_query: Query<(&Transform, &Launcher)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (launcher_transform, launcher) in launcher_query.iter() {
            let rotation = launcher_transform.rotation;
            let direction = rotation * Vec3::Y;
            let direction_2d = Vec2::new(direction.x, direction.y).normalize();
            let spawn_offset = direction * launcher.height as f32;
            let spawn_position = launcher_transform.translation + spawn_offset;

            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.5, 0.0),
                    custom_size: Some(Vec2::new(12.0, 12.0)),
                    ..default()
                },
                Transform::from_translation(spawn_position),
                Projectile {
                    velocity: direction_2d * launcher.projectile_speed as f32,
                    distance: 0.,
                    damage: 20.0,
                },
            ));
        }
    }
}

fn projectile_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Projectile)>) {
    for (mut transform, mut projectile) in query.iter_mut() {
        let movement = Vec2::new(
            projectile.velocity.x * time.delta_secs(),
            projectile.velocity.y * time.delta_secs(),
        );

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        projectile.distance += movement.length();
    }
}

fn projectile_collision(
    mut commands: Commands,
    mut explosion_events: EventWriter<ProjectileExplosionEvent>,
    mut destroy_uap_events: EventWriter<DestroyUapEvent>,
    mut score_events: EventWriter<ScoreEvent>,
    projectiles: Query<(Entity, &Transform, &Projectile, &Sprite)>,
    mut uaps: Query<(Entity, &Transform, &mut Uap, &Sprite)>,
) {
    for (projectile_entity, projectile_transform, projectile, projectile_sprite) in
        projectiles.iter()
    {
        for (uap_entity, uap_transform, mut uap, uap_sprite) in uaps.iter_mut() {
            let projectile_size = projectile_sprite
                .custom_size
                .unwrap_or(Vec2::new(12.0, 12.0));
            let uap_size = uap_sprite.custom_size.unwrap_or(Vec2::new(56.0, 24.0));
            let projectile_position = projectile_transform.translation.xy();
            let uap_position = uap_transform.translation.xy();
            let projectile_half = projectile_size / 2.0;
            let uap_half = uap_size / 2.0;
            let x_overlap =
                (projectile_position.x - uap_position.x).abs() < (projectile_half.x + uap_half.x);
            let y_overlap =
                (projectile_position.y - uap_position.y).abs() < (projectile_half.y + uap_half.y);

            if x_overlap && y_overlap {
                explosion_events.write(ProjectileExplosionEvent {
                    position: projectile_transform.translation,
                });
                uap.take_damage(
                    projectile.damage,
                    uap_entity,
                    uap_transform,
                    &mut destroy_uap_events,
                    &mut score_events,
                );
                commands.entity(projectile_entity).despawn();

                break;
            }
        }
    }
}

fn cleanup_projectiles(
    mut commands: Commands,
    mut explosion_events: EventWriter<ProjectileExplosionEvent>,
    query: Query<(Entity, &Transform, &Projectile)>,
) {
    for (entity, transform, projectile) in query.iter() {
        let auto_detonate = projectile.distance > 1000.0;

        if auto_detonate {
            explosion_events.write(ProjectileExplosionEvent {
                position: transform.translation,
            });
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_launcher(mut commands: Commands, query: Query<Entity, With<Launcher>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LauncherAssets {
    #[dependency]
    launcher: Handle<Image>,
}

impl FromWorld for LauncherAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            launcher: assets.load("images/launcher.png"),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LauncherCrankAssets {
    #[dependency]
    launcher_crank: Handle<Image>,
}

impl FromWorld for LauncherCrankAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            launcher_crank: assets.load("images/launcher_crank.png"),
        }
    }
}
