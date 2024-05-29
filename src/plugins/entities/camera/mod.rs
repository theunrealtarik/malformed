use bevy::prelude::*;

use super::player::*;
use crate::GameState;

#[derive(Component)]
pub struct MainCamera;

const CAMERA_STARTING_POSITIION: Vec2 = Vec2::new(0.0, 512.0);
const CAMERA_PLAYER_OFFSET: Vec2 = Vec2::new(512.0, 256.0);

#[derive(Bundle)]
pub struct MainCameraBundle {
    camera_2d: Camera2dBundle,
    tag: MainCamera,
    name: Name,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum Focus {
    Player,
    #[default]
    Menu,
}

impl MainCameraBundle {
    pub fn new() -> Self {
        Self {
            camera_2d: Camera2dBundle {
                transform: Transform::from_xyz(
                    CAMERA_STARTING_POSITIION.x,
                    CAMERA_STARTING_POSITIION.y,
                    0.0,
                ),
                ..Default::default()
            },
            tag: MainCamera,
            name: Name::new("Main Camera"),
        }
    }
}

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::setup)
            .init_state::<Focus>()
            .add_systems(
                Update,
                (
                    Self::follow_player
                        .run_if(in_state(Being::Alive))
                        .run_if(in_state(MovementType::Running)),
                    Self::set_focus_on_player.run_if(in_state(Focus::Menu)),
                )
                    .run_if(in_state(GameState::Resumed)),
            );
    }
}

impl GameCameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn(MainCameraBundle::new());
    }

    fn set_focus_on_player(
        mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
        mut next_foucs: ResMut<NextState<Focus>>,
        time: Res<Time>,
    ) {
        if let Ok(mut transform) = camera.get_single_mut() {
            if transform.translation.y >= 0.0 {
                transform.translation.y -= CAMERA_STARTING_POSITIION.y
                    * (1.0 - time.delta_seconds().powf(10f32.powf(-f32::exp(1.0))));
            } else {
                next_foucs.set(Focus::Player);
            }
        }
    }

    fn follow_player(
        player: Query<&Transform, With<Player>>,
        mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
        time: Res<Time>,
    ) {
        for player_transform in player.iter() {
            for mut camera_transform in camera.iter_mut() {
                let target = player_transform.translation
                    + Vec3::new(CAMERA_PLAYER_OFFSET.x, CAMERA_PLAYER_OFFSET.y, 0.0);

                let cam = &mut camera_transform.translation;
                let delta = target - *cam;

                *cam += delta * (1.0 - 0.05_f32.powf(time.delta_seconds()));
            }
        }
    }
}
