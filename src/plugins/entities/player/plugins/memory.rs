use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::na;
use bevy_tweening::*;

use super::super::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Memory {
    pub value: f32,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct SanityBar;

#[derive(Debug, Default, Component, Reflect)]
pub struct MemoryTimer(pub Timer);

pub(in super::super) struct PlayerMemoryPlugin;

impl Plugin for PlayerMemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MovementType::Running), Self::setup)
            .add_systems(FixedUpdate, Self::update)
            .add_systems(
                Update,
                Self::drain
                    .run_if(in_state(GameState::Resumed))
                    .run_if(in_state(MovementType::Running)),
            )
            .register_type::<Memory>()
            .register_type::<MemoryTimer>();
    }
}

impl PlayerMemoryPlugin {
    pub fn setup(mut commands: Commands, window: Query<&Window>) {
        let window = window.single();

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            WidthLens {
                start: 0.0,
                end: 100.0,
            },
        );

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
                    ..default()
                },
                ..default()
            })
            .insert(Animator::new(tween))
            .insert(Name::new("Memory Container"))
            .with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(window.width() / 4.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                        ..default()
                    })
                    .insert(Name::new("Memory"))
                    .with_children(|commands| {
                        commands
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..Default::default()
                                },
                                background_color: Color::WHITE.into(),
                                ..Default::default()
                            })
                            .insert(SanityBar);
                    });
            });
    }

    pub fn update(
        memory: Query<&Memory, With<Player>>,
        mut bar: Query<&mut Style, With<SanityBar>>,
    ) {
        let Ok(memory) = memory.get_single() else {
            return;
        };

        let Ok(mut bar) = bar.get_single_mut() else {
            return;
        };

        bar.width = Val::Percent(memory.value / PLAYER_MAX_MEMORY * 100.0);
    }

    pub fn drain(mut query: Query<(&mut Memory, &mut MemoryTimer)>, time: Res<Time>) {
        for (mut memory, mut timer) in query.iter_mut() {
            if timer.0.tick(time.delta()).just_finished() {
                memory.value = na::clamp(
                    memory.value - PLAYER_MEMORY_DRAINING_RATE,
                    0.0,
                    PLAYER_MAX_VELOCITY_X,
                );
            }
        }
    }
}

struct WidthLens {
    start: f32,
    end: f32,
}

impl Lens<Style> for WidthLens {
    fn lerp(&mut self, target: &mut Style, ratio: f32) {
        target.width = Val::Percent(self.start + (self.end - self.start) * ratio);
    }
}
