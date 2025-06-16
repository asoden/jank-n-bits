use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::{
        movement::{MovementController, ScreenWrap},
        uap_animation::UapAnimation,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Uap>();

    app.register_type::<UapAssets>();
    app.load_resource::<UapAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        uap_movement
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

pub fn uap(
    max_speed: f32,
    uap_assets: &UapAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let uap_animation = UapAnimation::new();
    (
        Name::new("UAP"),
        Uap { ..default() },
        Sprite {
            image: uap_assets.uap.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: uap_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec2::splat(1.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        uap_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
struct Uap {
    speed: f32,
    direction: f32,
    margin: f32,
}

impl Default for Uap {
    fn default() -> Self {
        Self {
            speed: 200.0,
            direction: 1.0,
            margin: 50.0,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct UapAssets {
    #[dependency]
    uap: Handle<Image>,
}

impl FromWorld for UapAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            uap: assets.load("images/uap.png"),
        }
    }
}

fn uap_movement(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Uap)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let half_width = window.width() / 2.0;

    for (mut transform, mut uap) in query.iter_mut() {
        let left_bound = -half_width + uap.margin;
        let right_bound = half_width - uap.margin;
        let movement = uap.speed * uap.direction * time.delta_secs();

        transform.translation.x += movement;

        if transform.translation.x >= right_bound && uap.direction > 0.0 {
            uap.direction = -1.0;
            transform.translation.x = right_bound;
        } else if transform.translation.x <= left_bound && uap.direction < 0.0 {
            uap.direction = 1.0;
            transform.translation.x = left_bound;
        }
    }
}
