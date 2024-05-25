use bevy::prelude::*;
use glib::Responsive;

use crate::plugins::game::prelude::*;

const INITIAL_PLAYER_VELOCITY_X: f32 = 0.0;
const INITIAL_PLAYER_ACCELERATION_X: f32 = 50.0;
const MAX_PLAYER_VELOCITY_X: f32 = 300.0;

enum PlayerAnimationState {
    Walking,
    Running,
    Rising,
    Falling,
}

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(Update, Self::run);
    }
}

impl PlayerPlugin {
    fn setup(mut commands: Commands, textures: Res<TextureAssets>, layouts: Res<SpriteLayouts>) {
        println!("loaded");
        commands
            .spawn(SpriteSheetBundle {
                texture: textures.player.clone(),
                atlas: TextureAtlas {
                    layout: layouts.player_layout.clone(),
                    index: 0,
                },
                transform: Transform {
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
}
