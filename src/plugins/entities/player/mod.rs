use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::debug::*;
use crate::plugins::game::ground::*;
use crate::*;

const PLAYER_SCALE_X: f32 = 2.0;
const PLAYER_SCALE_Y: f32 = 2.0;
const PLAYER_MASS: f32 = 85.0;
const PLAYER_JUMP_MAX_HEIGHT: f32 = 300.0;

const PLAYER_COLLIDER_WIDTH: f32 = 48.0;
const PLAYER_COLLIDER_HEIGHT: f32 = 36.0;

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
            Self::Rising => {
                Animation::new(DEFAULT_CYCLE_DELAY, Frame::range(30, 31), TimerMode::Once)
            }
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

#[derive(Component)]
pub struct PlayerChild;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .init_state::<PlayerAnimation>()
            .init_state::<PlayerControlable>()
            .add_plugins(EntityInspector::<Player>::default())
            .add_systems(
                Update,
                (
                    Self::set_animations,
                    Self::control_animations.run_if(in_state(GameAssetsState::Loaded)),
                    Self::jump.run_if(in_state(GameState::Game)),
                ),
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
                    translation: Vec3::new(0.0, -200.0, 10.0),
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
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(
                PLAYER_COLLIDER_WIDTH / 2.0,
                PLAYER_COLLIDER_HEIGHT / 2.0,
            ))
            .insert(AdditionalMassProperties::Mass(50.0))
            .insert(KinematicCharacterController::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Sleeping::disabled())
            .insert(Velocity::default())
            .insert(Responsive)
            .insert(Player)
            .with_children(|commands| {
                commands
                    .spawn(Collider::cuboid(2.0, 2.0))
                    .insert(TransformBundle::from(Transform::from_xyz(
                        0.0,
                        -1.0 * (PLAYER_COLLIDER_HEIGHT / 2.0 + 2.0),
                        0.0,
                    )))
                    .insert(Sensor)
                    .insert(Sleeping::disabled())
                    .insert(Name::new("Ground Check"))
                    .insert(Grounded::new(false))
                    .insert(PlayerChild);
            });
    }

    fn toggle_visibility(mut player: Query<&mut Visibility, With<Player>>) {
        let mut visibility = player.single_mut();
        *visibility = Visibility::Visible;
    }

    fn control_animations(
        mut commands: Commands,
        mut query: Query<(
            Entity,
            &mut PlayerAnimationController,
            Option<&mut Animation>,
        )>,
    ) {
        if query.is_empty() {
            return;
        }

        let (entity, controller, animation) = query.single_mut();

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

    fn set_animations(
        mut player: Query<(&Velocity, &mut PlayerAnimationController), With<Animation>>,
        player_children: Query<&Grounded, With<PlayerChild>>,
    ) {
        if player.is_empty() {
            return;
        }

        let (velocity, mut controller) = player.single_mut();
        let grounded = player_children.single();

        if velocity.linvel.y < -0.01 && !grounded.value {
            controller.curr_animation = PlayerAnimation::Falling;
        } else if velocity.linvel.y > 0.01 && !grounded.value {
            controller.curr_animation = PlayerAnimation::Rising;
        } else {
            controller.curr_animation = PlayerAnimation::Idle;
        }
    }

    fn jump(
        mut commands: Commands,
        player: Query<Entity, With<Player>>,
        children: Query<&Grounded, With<PlayerChild>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        if children.is_empty() {
            return;
        }

        let player = player.single();
        let grounded = children.single();

        if input.just_pressed(KeyCode::Space) && grounded.value {
            commands.entity(player).insert(ExternalImpulse {
                impulse: Vec2::new(0.0, 4.0 * 10_f32.powi(6)),
                torque_impulse: 100.0,
            });
        }
    }
}
