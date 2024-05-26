use bevy::prelude::*;

use crate::plugins::game::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
struct Layer(usize);

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

        for (depth, (name, texture)) in bg_images.iter().enumerate() {
            commands
                .spawn(SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform {
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Layer(depth))
                .insert(Name::new(name.to_string()))
                .insert(Responsive);
        }
    }
}
