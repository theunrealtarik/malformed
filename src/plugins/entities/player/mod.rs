use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::debug::*;
use crate::plugins::game::ground::*;
use crate::*;

const PLAYER_SCALE_X: f32 = 2.0;
const PLAYER_SCALE_Y: f32 = 2.0;
const PLAYER_MASS: f32 = 85.0;

const PLAYER_COLLIDER_WIDTH: f32 = 24.0;
const PLAYER_COLLIDER_HEIGHT: f32 = 36.0;

const PLAYER_COYOTE_JUMP_TIME: f32 = 0.35;
const PLAYER_JUMP_BUFFERING_TIME: f32 = 0.3;

const PLAYER_MAX_VELOCITY_X: f32 = 800.0;
const PLAYER_VELOCITY_BUMP: f32 = 150.0;
const PLAYER_JUMP_HEIGHT: f32 = 300.0;

const PLAYER_WALKING_TIMER: Duration = Duration::from_secs(10);

const INITIAL_PLAYER_VELOCITY_X: f32 = 80.0;
const INITIAL_PLAYER_ACCECLERATION_X: f32 = 50.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerChild;

#[derive(Component, Reflect, Default)]
pub struct AuxiliaryVelocity {
    pub value: Vec2,
}

#[derive(Component, Reflect, Default)]
pub struct AuxiliaryAcceleration {
    pub value: Vec2,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum MovementType {
    Running,
    #[default]
    Walking,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Being {
    Dead,
    #[default]
    Alive,
}

#[derive(Component, Reflect)]
pub struct Jump {
    coyote: f32,
    buffering: f32,
}

impl Default for Jump {
    fn default() -> Self {
        Self {
            coyote: 0.0,
            buffering: 0.0,
        }
    }
}

#[derive(Component)]
pub struct WalkingTimer(Timer);

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .init_state::<PlayerAnimation>()
            .init_state::<MovementType>()
            .add_systems(
                Update,
                (
                    Self::set_animations,
                    Self::control_animations.run_if(in_state(GameAssetsState::Loaded)),
                    (
                        (Self::movement, Self::jump).run_if(in_state(MovementType::Running)),
                        Self::update,
                    )
                        .run_if(in_state(GameState::Game)),
                ),
            )
            .add_systems(OnEnter(GameState::Game), Self::toggle_visibility)
            .init_state::<Being>()
            .register_type::<PlayerAnimation>()
            .register_type::<PlayerAnimationController>()
            .register_type::<AuxiliaryVelocity>()
            .register_type::<AuxiliaryAcceleration>()
            .register_type::<Jump>()
            .add_plugins(StateInspectorPlugin::<MovementType>::default());
    }
}

impl PlayerPlugin {
    pub(crate) fn setup(
        mut commands: Commands,
        textures: Res<TextureAssets>,
        layouts: Res<SpriteLayouts>,
    ) {
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
            .insert(AdditionalMassProperties::Mass(PLAYER_MASS))
            .insert(KinematicCharacterController::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Sleeping::disabled())
            .insert(Velocity::default())
            .insert(GravityScale(1.0))
            .insert(ReadMassProperties::default())
            .insert(Responsive)
            .insert(Player)
            .insert(AuxiliaryAcceleration::default())
            .insert(AuxiliaryVelocity {
                value: Vec2::new(INITIAL_PLAYER_VELOCITY_X, 0.0),
            })
            .insert(WalkingTimer(Timer::new(
                PLAYER_WALKING_TIMER,
                TimerMode::Once,
            )))
            .insert(Jump::default())
            .with_children(|commands| {
                commands
                    .spawn(Collider::cuboid(PLAYER_COLLIDER_WIDTH / 2.0, 2.0))
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

    fn update(
        mut query: Query<(&mut WalkingTimer, &mut AuxiliaryVelocity)>,
        mut next_controlable: ResMut<NextState<MovementType>>,
        time: Res<Time>,
    ) {
        let (mut timer, mut velocity) = query.single_mut();

        let tick = timer.0.tick(time.delta());
        let velocity_x = &mut velocity.value.x;

        if tick.just_finished() {
            next_controlable.set(MovementType::Running);
        }

        if tick.finished() && *velocity_x < PLAYER_VELOCITY_BUMP {
            *velocity_x +=
                (PLAYER_VELOCITY_BUMP - *velocity_x) * (1.0 - time.delta_seconds().powi(9));
        }
    }

    fn movement(
        mut query: Query<(&mut AuxiliaryVelocity, &mut AuxiliaryAcceleration), With<Player>>,
        time: Res<Time>,
    ) {
        if query.is_empty() {
            return;
        }

        let (mut velocity, mut acceleration) = query.single_mut();

        velocity.value.x += acceleration.value.x * time.delta_seconds();
        acceleration.value.x =
            INITIAL_PLAYER_ACCECLERATION_X * (1.0 - velocity.value.x / PLAYER_MAX_VELOCITY_X);
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
        mut player: Query<
            (
                &Velocity,
                &AuxiliaryVelocity,
                &mut GravityScale,
                &mut PlayerAnimationController,
            ),
            With<Animation>,
        >,
        player_children: Query<&Grounded, With<PlayerChild>>,
    ) {
        if player.is_empty() {
            return;
        }

        let (velocity, aux_velocity, mut gravity, mut controller) = player.single_mut();
        let grounded = player_children.single();

        if grounded.value {
            if aux_velocity.value.x == 0.0 {
                controller.curr_animation = PlayerAnimation::Idle;
            } else if aux_velocity.value.x != 0.0
                && aux_velocity.value.x < INITIAL_PLAYER_VELOCITY_X + PLAYER_VELOCITY_BUMP
            {
                controller.curr_animation = PlayerAnimation::Walking;
            } else {
                controller.curr_animation = PlayerAnimation::Running;
            }
        } else if velocity.linvel.y < -0.01 {
            controller.curr_animation = PlayerAnimation::Falling;
            gravity.0 = 1.0;
        } else if velocity.linvel.y > 0.01 {
            controller.curr_animation = PlayerAnimation::Rising;
            gravity.0 = 1.6;
        }
    }

    fn jump(
        mut commands: Commands,
        mut player: Query<(Entity, &mut Jump, &ReadMassProperties), With<Player>>,
        children: Query<&Grounded, With<PlayerChild>>,
        input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        rules: Res<RapierConfiguration>,
    ) {
        if children.is_empty() {
            return;
        }

        let (entity, mut jump, mass) = player.single_mut();
        let grounded = children.single().value;

        if grounded {
            jump.coyote = PLAYER_COYOTE_JUMP_TIME;
        } else {
            jump.coyote -= time.delta_seconds();
        }

        if input.just_pressed(KeyCode::Space) {
            jump.coyote = 0.0;
            jump.buffering = PLAYER_JUMP_BUFFERING_TIME;
        } else {
            jump.buffering -= time.delta_seconds();
        }

        if jump.buffering > 0.0 && jump.coyote > 0.0 {
            let impulse = Vec2::new(
                0.0,
                mass.get().mass * (PLAYER_JUMP_HEIGHT * rules.gravity.y * -2.0).sqrt(),
            );

            commands.entity(entity).insert(ExternalImpulse {
                impulse,
                torque_impulse: 100.0,
            });

            jump.buffering = 0.0;
        }
    }
}
