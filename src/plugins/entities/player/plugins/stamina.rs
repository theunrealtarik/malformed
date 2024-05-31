use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use super::super::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Stamina {
    pub value: f32,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct StaminaBar;

pub(in super::super) struct PlayerStaminaPlugin;

impl Plugin for PlayerStaminaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MovementType::Running), Self::setup)
            .add_systems(FixedUpdate, Self::update)
            .register_type::<Stamina>();
    }
}

impl PlayerStaminaPlugin {
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
            .insert(Name::new("Stamina Container"))
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
                    .insert(Name::new("Stamina"))
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
                            .insert(StaminaBar);
                    });
            });
    }

    pub fn update(
        stamina: Query<&Stamina, With<Player>>,
        mut stamina_bar: Query<&mut Style, With<StaminaBar>>,
    ) {
        let Ok(stamina) = stamina.get_single() else {
            return;
        };

        let Ok(mut stamina_bar) = stamina_bar.get_single_mut() else {
            return;
        };

        stamina_bar.width = Val::Percent(stamina.value / PLAYER_MAX_STAMINA * 100.0);
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
