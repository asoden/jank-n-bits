use bevy::prelude::*;

use crate::{app::uap::{uap, Uap, UapAssets}, screens::Screen, PausableSystems};

const MAX_UAPS: usize = 10;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(UapSpawnTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });
    app.add_systems(
        Update,
        (
            (spawn_uap)
                .chain()
                .run_if(in_state(Screen::Launchpad))
        )
            .in_set(PausableSystems),
    );
}

#[derive(Resource)]
struct UapSpawnTimer {
    timer: Timer,
}

fn spawn_uap(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<UapSpawnTimer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    uap_assets: Res<UapAssets>,
    uap_query: Query<&Uap>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() && uap_query.iter().len() < MAX_UAPS {
        // TODO: logic to spawn Uap
        commands.spawn((
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Launchpad),
            // children![
            //     uap(400.0, &uap_assets, &mut texture_atlas_layouts),
            // ]
        )).with_children(|parent| {
            parent.spawn(uap(400.0, &uap_assets, &mut texture_atlas_layouts));
        });
    }
}