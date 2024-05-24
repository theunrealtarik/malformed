use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;

use super::assets::{GameAssetsState, UiTextureAssets};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
#[allow(dead_code)]
enum GameState {
    #[default]
    Menu,
    Game,
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(StateInspectorPlugin::<GameState>::default())
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
    fn wait(mut commands: Commands, ui: Res<UiTextureAssets>, query: Query<Entity, With<Menu>>) {
        if query.is_empty() {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Menu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(798.0 / 2.0),
                                height: Val::Px(80.0 / 2.0),
                                align_self: AlignSelf::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(ui.pts.clone()),
                    ));
                });
        }
    }

    fn start(
        mut commands: Commands,
        query: Query<Entity, With<Menu>>,
        input: Res<ButtonInput<KeyCode>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        if query.is_empty() {
            return;
        }

        for entity in query.iter() {
            if input.just_pressed(KeyCode::Space) {
                commands.entity(entity).despawn_descendants();
                game_state.set(GameState::Game);
            }
        }
    }
}
