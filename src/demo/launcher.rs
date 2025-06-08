use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Launchpad), setup);
    app.add_systems(OnExit(Screen::Launchpad), despawn_launcher);
    app.add_systems(
        Update,
        (
            // setup.run_if(in_state(Screen::Launchpad).and(run_once)),
            (
                launcher_rotation,
                launcher_shooting,
                projectile_movement,
                cleanup_projectiles,
            )
                .chain()
                .run_if(in_state(Screen::Launchpad)),
        ),
    );
}

#[derive(Component)]
struct Launcher {
    rotation_speed: f32,
    projectile_speed: f32,
    height: f32,
}

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
}

fn setup(mut commands: Commands) {
    let launcher_width = 20.0;
    let launcher_height = 80.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.3),
            custom_size: Some(Vec2::new(launcher_width, launcher_height)),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
        Launcher {
            rotation_speed: 2.0,
            projectile_speed: 500.0,
            height: launcher_height,
        },
    ));
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
            let rotation_amount = rotation_direction * launcher.rotation_speed * time.delta_secs();
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
            let spawn_offset = direction * launcher.height;
            let spawn_position = launcher_transform.translation + spawn_offset;

            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.5, 0.0),
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                Transform::from_translation(spawn_position),
                Projectile {
                    velocity: direction_2d * launcher.projectile_speed,
                },
            ));
        }
    }
}

fn projectile_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation.x += projectile.velocity.x * time.delta_secs();
        transform.translation.y += projectile.velocity.y * time.delta_secs();
    }
}

fn cleanup_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y > 600.0
            || transform.translation.y < -600.0
            || transform.translation.x > 800.0
            || transform.translation.x < -800.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_launcher(mut commands: Commands, query: Query<Entity, With<Launcher>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
