use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::{debug::EntityDebugMenu, game::prelude::Velocity};
use crate::*;

const INITIAL_PLAYER_VELOCITY_X: f32 = 0.0;
const INITIAL_PLAYER_ACCELERATION_X: f32 = 50.0;
const MAX_PLAYER_VELOCITY_X: f32 = 300.0;

#[derive(Reflect, InspectorOptions, Default, Clone, Copy)]
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
            Self::Idle => Animation::new(Frame::range(0, 9)),
            Self::Walking => Animation::new(Frame::range(10, 17)),
            Self::Running => Animation::new(Frame::range(20, 27)),
            Self::Rising => Animation::new(Frame::range(30, 31)),
            Self::Falling => Animation::new(Frame::range(32, 32)),
        }
    }
}

#[derive(Component, Reflect, InspectorOptions, Default)]
#[reflect(InspectorOptions)]
pub struct PlayerAnimationController {
    animation: PlayerAnimation,
}

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(Update, Self::run)
            .add_systems(
                Update,
                EntityDebugMenu::inspector::<Player>.after(Self::setup),
            )
            .add_systems(
                Update,
                Self::init_animation.run_if(in_state(GameAssetsState::Loaded)),
            )
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
                    scale: Vec3::new(2.0, 2.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Acceleration::from(Vec3::new(
                INITIAL_PLAYER_ACCELERATION_X,
                0.0,
                0.0,
            )))
            .insert(Velocity::from(Vec3::new(
                INITIAL_PLAYER_VELOCITY_X,
                0.0,
                0.0,
            )))
            .insert(Name::new("Player"))
            .insert(PlayerAnimationController {
                animation: PlayerAnimation::Idle,
            })
            .insert(Collider::cuboid(24.0, 18.0))
            .insert(RigidBody::default())
            .insert(Responsive)
            .insert(Player);
    }

    fn run(mut query: Query<(&mut Acceleration, &mut Velocity, &mut Transform), With<Player>>) {
        if query.is_empty() {
            return;
        }

        let (mut acceleration, mut velocity, _) = query.single_mut();
        let velocity_ratio = velocity.x / MAX_PLAYER_VELOCITY_X;

        acceleration.x = INITIAL_PLAYER_ACCELERATION_X * (1.0 - velocity_ratio);

        if velocity.x >= MAX_PLAYER_VELOCITY_X {
            velocity.x = MAX_PLAYER_VELOCITY_X;
        }
    }

    fn init_animation(
        mut commands: Commands,
        query: Query<(Entity, &PlayerAnimationController), Without<Animation>>,
    ) {
        if query.is_empty() {
            return;
        }

        let (entity, controller) = query.single();

        commands
            .entity(entity)
            .insert(controller.animation.animation());
    }
}
