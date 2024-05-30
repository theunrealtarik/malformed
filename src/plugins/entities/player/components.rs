use crate::plugins::debug::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Jump {
    pub coyote: f32,
    pub buffering: f32,
    pub press: f32,
    pub rising: bool,
}

#[derive(Component, Default)]
pub(crate) struct Player;

#[derive(Component)]
pub struct PlayerGrounded;

#[derive(Component, Reflect, Default)]
pub(crate) struct AuxiliaryVelocity {
    pub value: Vec2,
}

#[derive(Component, Reflect, Default)]
pub struct AuxiliaryAcceleration {
    pub value: Vec2,
}

#[derive(Component, Default)]
pub struct WalkingTimer(pub Timer);

#[derive(Component, Default, Reflect)]
pub struct DialogTimer(pub Timer);

#[derive(Component, Reflect, InspectorOptions, Default)]
#[reflect(InspectorOptions)]
pub struct PlayerAnimationController {
    pub curr_animation: super::states::PlayerAnimation,
}
