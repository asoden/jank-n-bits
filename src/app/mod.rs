//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
mod fireworks;
pub mod launcher;
pub mod level;
mod movement;
pub mod player;
pub mod score;
pub mod uap;
mod uap_animation;
mod uap_spawner;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        launcher::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        fireworks::plugin,
        uap::plugin,
        uap_animation::plugin,
        uap_spawner::plugin,
        score::plugin,
    ));
}
