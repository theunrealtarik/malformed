use bevy::prelude::*;
use std::time::Duration;

pub const DEFAULT_CYCLE_DELAY: Duration = Duration::from_millis(70);

#[derive(Component, Debug)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<Frame>,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            timer: Timer::new(DEFAULT_CYCLE_DELAY, TimerMode::Repeating),
            frames,
        }
    }
}

#[derive(Component, Debug)]
pub struct Frame {
    pub duration: Duration,
    pub index: usize,
}

impl Frame {
    pub fn new(index: usize, duration: Duration) -> Self {
        Self { index, duration }
    }

    pub fn default(index: usize) -> Self {
        Self {
            index,
            duration: DEFAULT_CYCLE_DELAY,
        }
    }

    pub fn range(start: usize, end: usize) -> Vec<Self> {
        let mut frames = vec![];
        for index in start..=end {
            frames.push(Frame::default(index));
        }
        frames
    }
}

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::animate);
    }
}

impl GameAnimationPlugin {
    fn animate(mut query: Query<(&mut TextureAtlas, &mut Animation)>, time: Res<Time>) {
        for (mut atlas, mut animation) in query.iter_mut() {
            if animation.timer.tick(time.delta()).just_finished() {
                let current_idx = animation
                    .frames
                    .iter()
                    .position(|s| s.index == atlas.index)
                    .unwrap_or(0);

                let next_idx = (current_idx + animation.timer.times_finished_this_tick() as usize)
                    % animation.frames.len();

                let next_frame = &animation.frames[next_idx];
                let (index, duration) = (next_frame.index, next_frame.duration);

                atlas.index = index;
                animation.timer.set_duration(duration);
            }
        }
    }
}
