use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::BackgroundPlugin;

#[derive(Component)]
pub struct Platform;

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    tag: Platform,
}

impl PlatformBundle {
    fn new(color: Color, (x, y): (f32, f32), (w, h): (f32, f32)) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::new(w, h, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
            tag: Platform,
        }
    }
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup.after(BackgroundPlugin::setup));
    }
}

impl PlatformsPlugin {
    pub fn setup(mut commands: Commands, window: Query<&Window>) {
        let window = window.single();
        let height = window.height();
        let width = window.width();

        let left = width / -2.0;
        let bottom = height / -2.0;

        commands.spawn(PlatformBundle::new(
            Color::BLACK,
            (left + 100.0, 0.0),
            (200.0, height),
        ));
        commands.spawn(PlatformBundle::new(
            Color::BLACK,
            (0.0, bottom),
            (2000.0, 1000.0),
        ));
    }
}
