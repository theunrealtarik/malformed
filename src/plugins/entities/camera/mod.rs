use bevy::prelude::*;

use super::player::Player;
use crate::GameState;

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
pub struct MainCameraBundle {
    camera_2d: Camera2dBundle,
    tag: MainCamera,
    name: Name,
}

impl MainCameraBundle {
    pub fn new() -> Self {
        Self {
            camera_2d: Camera2dBundle::default(),
            tag: MainCamera,
            name: Name::new("Main Camera"),
        }
    }
}

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::setup).add_systems(
            Update,
            Self::follow_player.run_if(in_state(GameState::Game)),
        );
    }
}

impl GameCameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn(MainCameraBundle::new());
    }

    fn follow_player(
        player: Query<&Transform, With<Player>>,
        mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
        time: Res<Time>,
    ) {
        for player_transform in player.iter() {
            for mut camera_transform in camera.iter_mut() {
                let target = player_transform.translation;
                let cam = &mut camera_transform.translation;
                let delta = target - *cam;

                *cam += delta * (1.0 - 0.05_f32.powf(time.delta_seconds()));
            }
        }
    }
}
