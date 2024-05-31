use std::time::Duration;

use super::super::*;
use bevy::prelude::*;
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::*;

pub(in super::super) struct PlayerScorePlugin;

#[derive(Default, Debug, Reflect, Component)]
pub struct Score {
    pub value: f32,
}

#[derive(Component)]
pub struct ScoreLabel;

impl Plugin for PlayerScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MovementType::Running),
            Self::setup.after(PlayerPlugin::setup),
        )
        .add_systems(FixedUpdate, Self::update)
        .register_type::<Score>();
    }
}

impl PlayerScorePlugin {
    pub fn setup(
        mut commands: Commands,
        player: Query<Entity, With<Player>>,
        fonts: Res<FontsAssets>,
    ) {
        if player.is_empty() {
            return;
        }

        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            UiPositionLens {
                start: UiRect::top(Val::Percent(-100.0)),
                end: UiRect::top(Val::Percent(0.0)),
            },
        );

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
                    ..default()
                },
                ..default()
            })
            .insert(Animator::new(tween))
            .insert(Name::new("Score Container"))
            .with_children(|commands| {
                commands
                    .spawn(
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: fonts.vcr.clone(),
                                font_size: 100.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style::default()),
                    )
                    .insert(Label)
                    .insert(ScoreLabel);
            });
    }

    pub fn update(
        score: Query<&Score, With<Player>>,
        mut score_text: Query<&mut Text, With<ScoreLabel>>,
    ) {
        let Ok(score) = score.get_single() else {
            return;
        };

        let Ok(mut score_text) = score_text.get_single_mut() else {
            return;
        };

        let formated = format!("{}", score.value.round());
        score_text.sections[0].value = formated;
    }
}
