use crate::plugins::entities::player::*;
use crate::plugins::game::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

use glib::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Layer;

#[derive(Component, Reflect)]
struct Depth(usize);

#[derive(Bundle)]
struct LayerBundle {
    sprite: SpriteBundle,
    depth: Depth,
    name: Name,
}

impl LayerBundle {
    fn new(name: &'static str, texture: Handle<Image>, depth: usize) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(0.0, BACKGROUND_LAYER_Y, -1.0 * depth as f32),
                    scale: Vec3::new(BACKGROUND_LAYER_SACLE, BACKGROUND_LAYER_SACLE, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    anchor: Anchor::TopCenter,
                    ..Default::default()
                },
                ..Default::default()
            },
            depth: Depth(depth),
            name: Name::new(name),
        }
    }
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup);
        app.add_systems(
            Update,
            Self::update
                .run_if(in_state(GameState::Resumed))
                .run_if(in_state(Being::Alive)),
        );

        app.register_type::<Depth>();
    }
}

impl BackgroundPlugin {
    pub fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
        let bg_images = [
            ("Background Buildings", textures.bg_buildings_1.clone()),
            ("Background Clouds", textures.bg_cloud_1.clone()),
            ("Background Buildings", textures.bg_buildings_0.clone()),
            ("Background Clouds", textures.bg_cloud_0.clone()),
        ];

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(36.0 / 255.0, 43.0 / 255.0, 54.0 / 255.0),
                anchor: Anchor::TopCenter,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, PLATFORMS_MIN_Y, 1.0),
                scale: Vec3::new(4000.0, 2000.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });

        for (depth, (name, texture)) in bg_images.iter().enumerate() {
            commands
                .spawn(LayerBundle::new(name, texture.clone(), depth + 1))
                .insert(Layer)
                .insert(Responsive);
        }
    }

    fn update(
        player_velocity: Query<&AuxiliaryVelocity, With<Player>>,
        mut layers: Query<(&mut Transform, &Depth), With<Layer>>,
        time: Res<Time>,
    ) {
        if let Ok(velocity) = player_velocity.get_single() {
            for (mut transform, depth) in layers.iter_mut() {
                let frame = (transform.scale.x / 3.0) * BACKGROUND_IMAGE_WIDTH;
                if transform.translation.x <= frame * -1.0 {
                    transform.translation.x = frame;
                }

                transform.translation.x -=
                    (velocity.value.x / (depth.0 as f32)) * time.delta_seconds();
            }
        }
    }
}
