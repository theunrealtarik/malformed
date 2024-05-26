use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::debug::*;
use crate::*;

const PLAYER_SCALE_X: f32 = 2.0;
const PLAYER_SCALE_Y: f32 = 2.0;
const PLAYER_MASS: f32 = 85.0;
const PLAYER_JUMP_MAX_HEIGHT: f32 = 300.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum PlayerControlable {
    True,
    #[default]
    False,
}

#[derive(Reflect, InspectorOptions, Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reflect(InspectorOptions)]
enum PlayerAnimation {
    #[default]
    Idle,
    Walking,
    Running,
    Rising,
    Falling,
}

impl PlayerAnimation {
    fn animation(self) -> Animation {
        match self {
            Self::Idle => Animation::default(Frame::range(0, 9)),
            Self::Walking => Animation::default(Frame::range(10, 17)),
            Self::Running => Animation::default(Frame::range(20, 27)),
            Self::Rising => Animation::default(Frame::range(30, 31)),
            Self::Falling => {
                Animation::new(DEFAULT_CYCLE_DELAY, Frame::range(32, 32), TimerMode::Once)
            }
        }
    }
}

#[derive(Component, Reflect, InspectorOptions, Default)]
#[reflect(InspectorOptions)]
pub struct PlayerAnimationController {
    curr_animation: PlayerAnimation,
}

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .init_state::<PlayerAnimation>()
            .init_state::<PlayerControlable>()
            .add_plugins(EntityInspector::<Player>::default())
            .add_systems(
                Update,
                Self::control_animations.run_if(in_state(GameAssetsState::Loaded)),
            )
            .add_systems(OnEnter(GameState::Game), Self::toggle_visibility)
            .register_type::<PlayerAnimation>()
            .register_type::<PlayerAnimationController>();
    }
}

impl PlayerPlugin {
    fn setup(mut commands: Commands, textures: Res<TextureAssets>, layouts: Res<SpriteLayouts>) {
        commands
            .spawn(SpriteSheetBundle {
                texture: textures.player.clone(),
                atlas: TextureAtlas {
                    layout: layouts.player_layout.clone(),
                    index: 0,
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    scale: Vec3::new(PLAYER_SCALE_X, PLAYER_SCALE_Y, 0.0),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            })
            .insert(Name::new("Player"))
            .insert(PlayerAnimationController {
                curr_animation: PlayerAnimation::Walking,
            })
            .insert(Collider::cuboid(24.0, 18.0))
            .insert(RigidBody::Dynamic)
            .insert(KinematicCharacterController::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Responsive)
            .insert(Player);
    }

    fn toggle_visibility(mut player: Query<&mut Visibility, With<Player>>) {
        if let Ok(mut visibility) = player.get_single_mut() {
            *visibility = Visibility::Visible;
        }
    }

    fn control_animations(
        mut commands: Commands,
        mut query: Query<(Entity, &PlayerAnimationController, Option<&mut Animation>)>,
    ) {
        if query.is_empty() {
            return;
        }

        if let Ok((entity, controller, animation)) = query.get_single_mut() {
            match animation {
                Some(mut animation) => {
                    let new_animation = controller.curr_animation.animation();
                    if *animation != new_animation {
                        *animation = new_animation;
                    }
                }
                None => {
                    commands
                        .entity(entity)
                        .insert(controller.curr_animation.animation());
                }
            }
        }
    }

    fn start_running(
        cabinet: Query<&Transform, With<super::terrain::Cabinet>>,
        player: Query<&Transform, With<Player>>,
    ) {
    }

    fn jump() {}
}
