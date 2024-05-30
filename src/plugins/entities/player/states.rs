use crate::{plugins::debug::*, Animation, Frame, DEFAULT_CYCLE_DELAY};
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum MovementType {
    Running,
    #[default]
    Walking,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Being {
    Dead,
    #[default]
    Alive,
}

#[derive(Reflect, InspectorOptions, Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(InspectorOptions)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    Walking,
    Running,
    Rising,
    Falling,
}

impl PlayerAnimation {
    pub fn animation(self) -> Animation {
        match self {
            Self::Idle => Animation::default(Frame::range(0, 9)),
            Self::Walking => Animation::default(Frame::range(10, 17)),
            Self::Running => Animation::default(Frame::range(20, 27)),
            Self::Rising => {
                Animation::new(DEFAULT_CYCLE_DELAY, Frame::range(30, 31), TimerMode::Once)
            }
            Self::Falling => {
                Animation::new(DEFAULT_CYCLE_DELAY, Frame::range(32, 32), TimerMode::Once)
            }
        }
    }
}
