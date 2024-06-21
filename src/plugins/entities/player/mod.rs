#![allow(clippy::all)]

use std::time::Duration;

use glib::*;

mod components;
mod plugins;
mod states;

pub use components::*;
pub use plugins::*;
pub use states::*;

use bevy::prelude::*;
use bevy_rapier2d::na;
use bevy_rapier2d::prelude::*;

use crate::plugins::game::ground::*;
use crate::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // states
        app.init_state::<PlayerAnimation>()
            .init_state::<MovementType>()
            .init_state::<Being>();
        // systems
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(OnEnter(GameState::Resumed), Self::setup_dialog_text)
            .add_systems(
                Update,
                (
                    Self::set_animations,
                    Self::update,
                    Self::control_animations.run_if(in_state(GameAssetsState::Loaded)),
                    (Self::movement, Self::jump, Self::collect)
                        .run_if(in_state(MovementType::Running))
                        .run_if(in_state(Being::Alive)),
                )
                    .run_if(in_state(GameState::Resumed)),
            )
            .add_systems(Update, Self::being.run_if(in_state(GameState::Resumed)));
        // types
        app.register_type::<PlayerAnimation>()
            .register_type::<PlayerAnimationController>()
            .register_type::<AuxiliaryVelocity>()
            .register_type::<AuxiliaryAcceleration>()
            .register_type::<WalkingTimer>()
            .register_type::<Jump>();
        // plugins
        app.add_plugins(PlayerMemoryPlugin);
        app.add_plugins(PlayerScorePlugin);
    }
}

impl PlayerPlugin {
    pub(crate) fn setup(
        mut commands: Commands,
        textures: Res<TextureAssets>,
        layouts: Res<SpriteLayouts>,
    ) {
        commands
            .spawn(PlayerBundle::new(
                textures.player.clone(),
                layouts.player_layout.clone(),
                0,
            ))
            .insert(MemoryTimer(Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating,
            )))
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
                    .insert(PlayerGrounded);
            });
    }

    fn update(
        mut query: Query<(&mut WalkingTimer, &mut AuxiliaryVelocity)>,
        mut next_controlable: ResMut<NextState<MovementType>>,
        time: Res<Time>,
    ) {
        let Ok((mut timer, mut velocity)) = query.get_single_mut() else {
            return;
        };

        let tick = timer.0.tick(time.delta());
        let velocity_x = &mut velocity.value.x;

        if tick.just_finished() {
            next_controlable.set(MovementType::Running);
        }

        if tick.finished() && *velocity_x < PLAYER_VELOCITY_BUMP {
            *velocity_x +=
                (PLAYER_VELOCITY_BUMP - *velocity_x) * (1.0 - time.delta_seconds().powi(12));
        }
    }

    fn being(
        mut commands: Commands,
        mut next_being: ResMut<NextState<Being>>,
        player: Query<(Entity, &Transform, &Memory), With<Player>>,
    ) {
        let Ok((entity, transform, memory)) = player.get_single() else {
            return;
        };

        let mut die = || {
            next_being.set(Being::Dead);
            commands.entity(entity).despawn_recursive();
        };

        if transform.translation.y < PLATFORMS_MIN_Y || memory.value == 0.0 {
            die();

            #[cfg(feature = "bsod", windows)]
            {
                glib::bsod::bsod();
            }
        }
    }

    fn movement(
        mut query: Query<
            (
                &mut AuxiliaryVelocity,
                &mut AuxiliaryAcceleration,
                &mut Score,
            ),
            With<Player>,
        >,
        time: Res<Time>,
    ) {
        if query.is_empty() {
            return;
        }

        let (mut velocity, mut acceleration, mut score) = query.single_mut();

        velocity.value.x += acceleration.value.x * time.delta_seconds();
        acceleration.value.x =
            PLAYER_INIT_ACCELERATION_X * (1.0 - velocity.value.x / PLAYER_MAX_VELOCITY_X);

        score.value += (velocity.value.x / 100f32) * time.delta_seconds();
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
        player_children: Query<&Grounded, With<PlayerGrounded>>,
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
                && aux_velocity.value.x < PLAYER_INIT_VELOCITY_X + PLAYER_VELOCITY_BUMP
            {
                controller.curr_animation = PlayerAnimation::Walking;
            } else {
                controller.curr_animation = PlayerAnimation::Running;
            }
        } else if velocity.linvel.y < -0.01 {
            controller.curr_animation = PlayerAnimation::Falling;
            *gravity = GravityScale(PLAYER_FALL_GRAVITY);
        } else if velocity.linvel.y > 0.01 {
            controller.curr_animation = PlayerAnimation::Rising;
            *gravity = GravityScale(PLAYER_RISE_GRAVITY);
        }
    }

    fn jump(
        mut commands: Commands,
        mut player: Query<
            (
                Entity,
                &ReadMassProperties,
                &mut Velocity,
                &AuxiliaryVelocity,
                &mut Jump,
                &mut GravityScale,
            ),
            With<Player>,
        >,
        children: Query<&Grounded, With<PlayerGrounded>>,
        input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        rules: Res<RapierConfiguration>,
    ) {
        if children.is_empty() {
            return;
        }

        let (entity, mass, mut velocity, _, mut jump, mut gravity) = player.single_mut();
        let grounded = children.single().value;

        if grounded {
            jump.coyote = PLAYER_COYOTE_JUMP_TIME;
            *gravity = GravityScale(1.0);
        } else {
            jump.coyote -= time.delta_seconds();
        }

        if input.just_pressed(KeyCode::Space) {
            jump.coyote = 0.0;
            jump.press = 0.0;
            jump.buffering = PLAYER_JUMP_BUFFERING_TIME;
        } else {
            jump.buffering -= time.delta_seconds();
        }

        let jump_magnitude = mass.get().mass
            * (PLAYER_JUMP_HEIGHT * -2.0 * rules.gravity.y / PLAYER_FALL_GRAVITY).sqrt();
        if jump.buffering > 0.0 && jump.coyote > 0.0 {
            commands.entity(entity).insert(ExternalImpulse {
                impulse: Vec2::new(0.0, jump_magnitude),
                torque_impulse: 0.0,
            });

            jump.buffering = 0.0;
            jump.rising = true;
        }

        if jump.rising {
            jump.press += time.delta_seconds();

            if input.just_released(KeyCode::Space) {
                if jump.press < PLAYER_JUMP_WINDOW {
                    jump.press = 0.0;
                    commands.entity(entity).insert(ExternalImpulse {
                        impulse: Vec2::new(0.0, -1.0 * f32::exp(-0.9) * jump_magnitude),
                        torque_impulse: 0.0,
                    });
                }
                *gravity = GravityScale(PLAYER_FALL_GRAVITY);
            }

            if velocity.linvel.y < 0.0 {
                velocity.linvel.y -= velocity.linvel.y * time.delta_seconds() * 2.0;
                jump.rising = false;
            }
        }
    }

    pub fn restart(
        mut query: Query<&mut AuxiliaryVelocity, With<Player>>,
        mut next_movement: ResMut<NextState<MovementType>>,
    ) {
        if query.is_empty() {
            return;
        }

        let mut velocity = query.single_mut();
        *velocity = AuxiliaryVelocity {
            value: Vec2::new(PLAYER_RESPAWN_VELOCITY, 0.0),
        };

        next_movement.set(MovementType::Running);
    }

    pub fn setup_dialog_text(mut commands: Commands, fonts: Res<FontsAssets>) {
        commands
            .spawn(
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.vcr.clone(),
                        font_size: 34.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::End,
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..Default::default()
                }),
            )
            .insert(Name::new("Self Dialog"))
            .insert(Dialog::new(
                DIALOG_LINES
                    .iter()
                    .map(|(content, duration)| Line::new(content, *duration))
                    .collect::<Vec<Line>>(),
            ));
    }

    pub fn collect(
        mut commands: Commands,
        mut player: Query<(Entity, &mut Memory), With<Player>>,
        bytes: Query<Entity, With<Byte>>,
        ctx: Res<RapierContext>,
    ) {
        if player.is_empty() || bytes.is_empty() {
            return;
        }

        let (player, mut memory) = player.single_mut();

        for byte in bytes.iter() {
            if ctx.intersection_pair(byte, player) == Some(true) {
                commands.entity(byte).despawn_recursive();
                memory.value = na::clamp(memory.value + PLAYER_MEMORY_REGEN_RATE, 0.0, 100.0);
            }
        }
    }
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    pub spritesheet: SpriteSheetBundle,
    pub name: Name,
    pub animation_controller: PlayerAnimationController,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub jump: Jump,
    pub mass_properties: AdditionalMassProperties,
    pub read_mass_properties: ReadMassProperties,
    pub locked_axes: LockedAxes,
    pub sleeping: Sleeping,
    pub velocity: Velocity,
    pub auxiliary_velocity: AuxiliaryVelocity,
    pub auxiliary_acceleration: AuxiliaryAcceleration,
    pub gravity_scale: GravityScale,
    pub responsive: Responsive,
    pub tag: Player,
    pub walking_timer: WalkingTimer,
    // stats
    pub memory: Memory,
    pub score: Score,
}

impl PlayerBundle {
    fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>, index: usize) -> Self {
        Self {
            name: Name::new("Player"),
            spritesheet: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas { layout, index },
                transform: Transform {
                    translation: Vec3::new(-40.0, 40.0, 10.0),
                    scale: Vec3::new(PLAYER_SCALE_X, PLAYER_SCALE_Y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // physics related
            rigidbody: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(PLAYER_COLLIDER_WIDTH / 2.0, PLAYER_COLLIDER_HEIGHT / 2.0),
            sleeping: Sleeping::disabled(),
            gravity_scale: GravityScale(1.0),
            mass_properties: AdditionalMassProperties::Mass(PLAYER_MASS),
            read_mass_properties: ReadMassProperties::default(),
            // virtual movement
            auxiliary_velocity: AuxiliaryVelocity {
                value: Vec2::new(PLAYER_INIT_VELOCITY_X, 0.0),
            },
            walking_timer: WalkingTimer(Timer::new(PLAYER_WALKING_TIMER, TimerMode::Once)),
            // stats
            memory: Memory {
                value: PLAYER_MAX_MEMORY,
            },
            ..Default::default()
        }
    }
}
