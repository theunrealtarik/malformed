use bevy::{prelude::*, sprite::Anchor};

use crate::plugins::entities::player::*;
use crate::plugins::game::prelude::*;

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

const BACKGROUND_IMAGE_WIDTH: f32 = 4608.0;
// const BACKGROUND_IMAGE_HEIGHT: f32 = 512.0;

impl LayerBundle {
    fn new(name: &'static str, texture: Handle<Image>, depth: usize) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_xyz(0.0, 0.0, depth as f32),
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
            ("Clouds", textures.bg_cloud_0.clone()),
            ("Buildings", textures.bg_buildings_0.clone()),
            ("Clouds", textures.bg_cloud_1.clone()),
            ("Buildings", textures.bg_buildings_1.clone()),
        ];

        commands
            .spawn(Background)
            .insert(Name::new("Background"))
            .insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(0.0, 512.0, 0.0),
                    scale: Vec3::new(3.0, 3.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(InheritedVisibility::default())
            .with_children(|commands| {
                for (depth, (name, texture)) in bg_images.iter().enumerate() {
                    commands
                        .spawn(LayerBundle::new(
                            name,
                            texture.clone(),
                            bg_images.len() - depth,
                        ))
                        .insert(Layer)
                        .insert(Responsive);
                }
            });
    }

    fn update(
        player_velocity: Query<&AuxiliaryVelocity, With<Player>>,
        mut layers: Query<(&mut Transform, &Depth), With<Layer>>,
        time: Res<Time>,
    ) {
        if let Ok(velocity) = player_velocity.get_single() {
            for (mut transform, depth) in layers.iter_mut() {
                let frame = (1.0 / 3.0) * BACKGROUND_IMAGE_WIDTH;
                if transform.translation.x <= frame * -1.0 {
                    transform.translation.x = frame;
                }

                transform.translation.x -=
                    (velocity.value.x / (depth.0 as f32)) * time.delta_seconds();
            }
        }
    }
}
