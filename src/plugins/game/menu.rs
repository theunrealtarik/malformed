use bevy::prelude::*;

use super::assets::{FontsAssets, GameAssetsState};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                Self::wait
                    .run_if(in_state(GameState::Menu))
                    .run_if(in_state(GameAssetsState::Loaded)),
            )
            .add_systems(Update, Self::start.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
struct Menu;

impl GameMenuPlugin {
    fn wait(mut commands: Commands, fonts: Res<FontsAssets>, query: Query<Entity, With<Menu>>) {
        if query.is_empty() {
            commands.spawn((
                TextBundle::from_section(
                    "space it to start it",
                    TextStyle {
                        font: fonts.vcr.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
                Menu,
            ));
        }
    }

    fn start(
        mut query: Query<(&Text, &mut Visibility), With<Menu>>,
        mut game_state: ResMut<NextState<GameState>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        if query.is_empty() {
            return;
        }

        for (_, mut visibility) in query.iter_mut() {
            if input.just_pressed(KeyCode::Space) {
                *visibility = Visibility::Hidden;
                game_state.set(GameState::Game);
            }
        }
    }
}
