use bevy::{prelude::*, sprite::Anchor};

use crate::plugins::game::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
struct Depth(usize);

#[derive(Bundle)]
struct Layer {
    sprite: SpriteBundle,
    depth: Depth,
    name: Name,
    responsive: Responsive,
}

impl Layer {
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
            responsive: Responsive,
        }
    }
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup);
    }
}

impl BackgroundPlugin {
    pub fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
        let bg_images = [
            ("Clouds", textures.bg_cloud_0.clone()),
            ("Buildings", textures.bg_buildings_0.clone()),
            ("Cloud", textures.bg_cloud_1.clone()),
            ("Buildings", textures.bg_buildings_1.clone()),
        ];

        commands
            .spawn(Background)
            .insert(Name::new("Background"))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 512.0, 0.0)))
            .insert(InheritedVisibility::default())
            .with_children(|commands| {
                for (depth, (name, texture)) in bg_images.iter().enumerate() {
                    commands.spawn(Layer::new(name, texture.clone(), depth));
                }
            });
    }
}
