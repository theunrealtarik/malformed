use bevy::prelude::*;

use crate::{GameAssetsState, GameState, TextureAssets};

#[derive(Component)]
struct Menu;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                Self::wait
                    .run_if(in_state(GameState::Paused))
                    .run_if(in_state(GameAssetsState::Loaded)),
            )
            .add_systems(Update, Self::start.run_if(in_state(GameState::Paused)));
    }
}

impl GameMenuPlugin {
    fn wait(
        mut commands: Commands,
        textures: Res<TextureAssets>,
        query: Query<Entity, With<Menu>>,
    ) {
        if query.is_empty() {
            commands
                .spawn(SpriteBundle {
                    texture: textures.pts.clone(),
                    transform: Transform::from_xyz(0.0, 512.0, 0.0),
                    ..Default::default()
                })
                .insert(Menu);
        }
    }

    fn start(
        mut query: Query<&mut Visibility, With<Menu>>,
        mut game_state: ResMut<NextState<GameState>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        if query.is_empty() {
            return;
        }

        for mut visibility in query.iter_mut() {
            if input.just_pressed(KeyCode::Space) {
                *visibility = Visibility::Hidden;
                game_state.set(GameState::Resumed);
            }
        }
    }
}
