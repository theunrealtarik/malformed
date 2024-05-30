use std::time::Duration;

use crate::plugins::debug::*;
use crate::{GameAssetsState, GameState};
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone, Debug, InspectorOptions)]
pub struct Dialog {
    pub index: usize,
    pub lines: Vec<Line>,
}

impl Dialog {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { index: 0, lines }
    }
}

#[derive(Default, Reflect, Clone, Debug)]
pub struct Line {
    content: String,
    timer: Timer,
}

impl Line {
    pub fn new(content: &'static str, duration: Duration) -> Self {
        Self {
            content: String::from(content),
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::play_dialogs
                .run_if(in_state(GameAssetsState::Loaded))
                .run_if(in_state(GameState::Resumed)),
        )
        .register_type::<Line>()
        .register_type::<Dialog>();
    }
}

impl DialogPlugin {
    fn play_dialogs(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Dialog, &mut Text)>,
        time: Res<Time>,
    ) {
        if query.is_empty() {
            return;
        }

        for (entity, mut dialog, mut text) in query.iter_mut() {
            let index = dialog.index;
            let curr_line = &mut dialog.lines[index];
            let curr_timer = &mut curr_line.timer;

            text.sections[0].value.clone_from(&curr_line.content);

            if curr_timer.tick(time.delta()).just_finished() {
                dialog.index =
                    (index + curr_timer.times_finished_this_tick() as usize) % dialog.lines.len();

                if index == dialog.lines.len() - 1 {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
