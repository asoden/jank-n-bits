//! Screen for creating firework bits.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::Val::*,
};

use crate::{Pause, menus::Menu, screens::Screen, theme::widget};

const WORKSHOP_TILE_WIDTH: f32 = 64.;
const WORKSHOP_COLUMNS: f32 = 8.;
const WORKSHOP_ROWS: f32 = 8.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Workshop), spawn_workshop);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Workshop)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Workshop)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Workshop), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Workshop)),
    );
}

fn spawn_workshop(mut commands: Commands, asset_server: Res<AssetServer>) {
    let workspace_sidebar = asset_server.load("images/workspace-panel.png");
    let workspace_texture = asset_server.load_with_settings(
        "images/workspace.png",
        |settings: &mut ImageLoaderSettings| {
            // Need to use nearest filtering to avoid bleeding between the slices with tiling
            settings.sampler = ImageSampler::nearest();
        },
    );

    let slicer = TextureSlicer {
        border: BorderRect::all(WORKSHOP_TILE_WIDTH),
        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(0.),
                row_gap: Val::Px(0.),
                ..default()
            },
            StateScoped(Screen::Workshop),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: workspace_sidebar.clone(),
                    image_mode: NodeImageMode::Sliced(slicer.clone()),
                    ..default()
                },
                Node {
                    width: Val::Px(128.),
                    height: Val::Px(64. * WORKSHOP_ROWS),
                    ..default()
                },
                StateScoped(Screen::Workshop),
            ));
            parent.spawn((
                ImageNode {
                    image: workspace_texture.clone(),
                    image_mode: NodeImageMode::Sliced(slicer.clone()),
                    ..default()
                },
                Node {
                    width: Val::Px(64. * WORKSHOP_COLUMNS),
                    height: Val::Px(64. * WORKSHOP_ROWS),
                    ..default()
                },
                StateScoped(Screen::Workshop),
            ));
        });

    commands.spawn((
        widget::button("Launch bits!", launch_bits),
        StateScoped(Screen::Workshop),
    ));
}

fn launch_bits(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Launchpad);
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
