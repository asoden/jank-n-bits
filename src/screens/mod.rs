//! The game's main screen states and transitions between them.

mod gameplay;
mod launchpad;
mod loading;
mod splash;
mod title;
mod workshop;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        workshop::plugin,
        launchpad::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
    Workshop,
    Launchpad,
}
