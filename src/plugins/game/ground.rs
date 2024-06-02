use std::marker::PhantomData;

use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Ground;

#[derive(Component, Reflect, PartialEq, Eq)]
pub struct Grounded {
    pub value: bool,
}

impl Grounded {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Default)]
pub struct GameGroundCheckPlugin<G: Component> {
    marker: PhantomData<G>,
}

impl<G> Plugin for GameGroundCheckPlugin<G>
where
    G: Component,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::check_ground.run_if(in_state(GameState::Resumed)),
        )
        .register_type::<Grounded>();
    }
}

impl<G> GameGroundCheckPlugin<G>
where
    G: Component,
{
    fn check_ground(
        mut checkers: Query<(Entity, &mut Grounded)>,
        grounds: Query<Entity, With<G>>,
        ctx: Res<RapierContext>,
    ) {
        for (checker, mut state) in checkers.iter_mut() {
            for ground in grounds.iter() {
                let result = ctx.intersection_pair(ground, checker);
                if result == Some(true) {
                    *state = Grounded::new(true);
                    return;
                } else {
                    *state = Grounded::new(false);
                }
            }
        }
    }
}
