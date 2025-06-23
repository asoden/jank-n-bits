use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Event)]
pub struct ScoreEvent {
    pub score_to_add: usize,
}

#[derive(Component)]
struct Scoreboard {
    score: usize,
}

impl Scoreboard {
    fn update_score(&mut self, points: usize) {
        self.score += points;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ScoreEvent>();

    app.add_systems(OnEnter(Screen::Launchpad), spawn_scoreboard);
    app.add_systems(OnExit(Screen::Launchpad), despawn_scoreboard);
    app.add_systems(
        Update,
        ((
            update_score
        )
            .chain()
            .run_if(in_state(Screen::Launchpad)),),
    );
}

pub fn scoreboard() -> impl Bundle {
    (
        Scoreboard {
            score: 0,
        },
    )
}

fn spawn_scoreboard(
    commands: Commands,
) {
    println!("spawning scoreboard...");
}

fn update_score(
    // commands: Commands,
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: Single<&mut Scoreboard>,
    // mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    for event in score_events.read() {
        println!("updating score by {} points...", event.score_to_add);
        scoreboard.update_score(event.score_to_add);
        println!("current score: {}", scoreboard.score);
    }
}

fn despawn_scoreboard(
    commands: Commands,
) {
    println!("despawning scoreboard...");
}
