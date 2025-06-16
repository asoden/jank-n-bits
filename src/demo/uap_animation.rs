use bevy::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    demo::{movement::MovementController, uap::UapAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<UapAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (update_animation_movement, update_animation_atlas)
                .chain()
                .run_if(resource_exists::<UapAssets>)
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

fn update_animation_movement(mut uap_query: Query<(&MovementController, &mut UapAnimation)>) {
    for (controller, mut animation) in &mut uap_query {
        let animation_state = if controller.intent == Vec2::ZERO {
            UapAnimationState::Idling
        } else {
            UapAnimationState::Flying
        };
        animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut UapAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&UapAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UapAnimation {
    timer: Timer,
    frame: usize,
    state: UapAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum UapAnimationState {
    Idling,
    Flying,
}

impl UapAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 4;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(100);
    /// The number of Flying frames.
    const FLYING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const FLYING_INTERVAL: Duration = Duration::from_millis(200);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: UapAnimationState::Idling,
        }
    }

    fn flying() -> Self {
        Self {
            timer: Timer::new(Self::FLYING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: UapAnimationState::Flying,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                UapAnimationState::Idling => Self::IDLE_FRAMES,
                UapAnimationState::Flying => Self::FLYING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: UapAnimationState) {
        if self.state != state {
            match state {
                UapAnimationState::Idling => *self = Self::idling(),
                UapAnimationState::Flying => *self = Self::flying(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            UapAnimationState::Idling => self.frame,
            UapAnimationState::Flying => self.frame,
        }
    }
}
