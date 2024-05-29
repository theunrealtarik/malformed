use crate::plugins::debug::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    Paused,
    Resumed,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum GameAssetsState {
    #[default]
    Pending,
    Loaded,
}
