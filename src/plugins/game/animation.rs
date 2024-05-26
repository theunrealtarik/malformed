use bevy::prelude::*;
use std::time::Duration;

use crate::plugins::debug::*;

pub const DEFAULT_CYCLE_DELAY: Duration = Duration::from_millis(70);

#[derive(Component, Debug, InspectorOptions, Reflect)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<Frame>,
}

impl PartialEq for Animation {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames
    }
}

#[cfg(test)]
mod test_anim {
    use crate::{Animation, Frame};

    #[test]
    fn is_same_animation() {
        let some_anim = Animation::default(Vec::new());
        let other_anim = Animation::default(Vec::new());
        assert!(some_anim == other_anim);
    }

    #[test]
    fn not_same_animation() {
        let some_anim = Animation::default(vec![Frame::default(0), Frame::default(1)]);
        let other_anim = Animation::default(vec![Frame::default(1), Frame::default(2)]);
        assert!(some_anim != other_anim);
    }
}

impl Animation {
    pub fn new(duration: Duration, frames: Vec<Frame>, mode: TimerMode) -> Self {
        Self {
            timer: Timer::new(duration, mode),
            frames,
        }
    }

    pub fn default(frames: Vec<Frame>) -> Self {
        Self::new(DEFAULT_CYCLE_DELAY, frames, TimerMode::Repeating)
    }
}

#[derive(Component, Debug, PartialEq, Eq, Reflect)]
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
        app.register_type::<Animation>();
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
