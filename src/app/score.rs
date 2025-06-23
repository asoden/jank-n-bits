use bevy::{prelude::*, ui::Val::*};

use crate::{screens::Screen, theme::widget};

#[derive(Event)]
pub struct ScoreEvent {
    pub score_to_add: usize,
}

#[derive(Component)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct LiveScore;

#[derive(Component)]
struct ScoreboardGrid;

impl Scoreboard {
    fn update_score(&mut self, points: usize) {
        self.score += points;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ScoreEvent>();

    app.add_systems(
        OnEnter(Screen::Launchpad),
        (init_score, spawn_scoreboard).chain(),
    );
    app.add_systems(OnExit(Screen::Launchpad), despawn_scoreboard);
    app.add_systems(
        Update,
        ((update_score).chain().run_if(in_state(Screen::Launchpad)),),
    );
}

pub fn score() -> impl Bundle {
    (Scoreboard { score: 0 },)
}

fn init_score(mut commands: Commands) {
    commands.spawn(score());
}

fn spawn_scoreboard(mut commands: Commands, scoreboard: Single<&Scoreboard>) {
    commands.spawn((
        Name::new("Scoreboard Grid"),
        GlobalZIndex(2),
        Node {
            justify_self: JustifySelf::End,
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 200.0),
            ..default()
        },
        children![
            (
                widget::label("Score:"),
                Node {
                    justify_self: JustifySelf::Start,
                    ..default()
                },
            ),
            (
                widget::label(scoreboard.score.to_string()),
                Node {
                    justify_self: JustifySelf::Start,
                    ..default()
                },
                LiveScore,
            )
        ],
        ScoreboardGrid,
    ));
}

fn update_score(
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: Single<&mut Scoreboard>,
    mut score: Single<&mut Text, With<LiveScore>>,
) {
    for event in score_events.read() {
        scoreboard.update_score(event.score_to_add);
        score.0 = scoreboard.score.to_string();
    }
}

fn despawn_scoreboard(
    mut commands: Commands,
    scoreboard: Single<Entity, With<Scoreboard>>,
    grid: Single<Entity, With<ScoreboardGrid>>,
) {
    commands.entity(*scoreboard).despawn();
    commands.entity(*grid).despawn();
}
