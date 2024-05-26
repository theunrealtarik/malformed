use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;

use crate::*;
use plugins::debug::*;

const PLATFORMS_MAX_Y: f32 = -640.0;
const PLATFORMS_MIN_Y: f32 = PLATFORMS_MAX_Y - 128.0;
const PLATFORMS_MAX_SPACING: f32 = 100.0;
const PLATFORMS_MIN_SPACING: f32 = 50.0;
const PLATFORMS_MAX_WIDTH: f32 = 1000.0;
const PLATFORMS_MIN_WIDTH: f32 = 500.0;
const PLATFORMS_HEIGHT: f32 = 1000.0;

const WORLD_MAX_PLATFORMS: u8 = 5;

const RTE_X: f32 = 0.0;
const RTE_Y: f32 = -269.5;

#[derive(Component)]
pub struct Cabinet;

#[derive(Component)]
pub struct Door;

#[derive(Resource, Default, Reflect, InspectorOptions)]
struct WorldPlatforms {
    last_platform: Platform,
    count: u8,
}

#[derive(Component, Reflect, Clone, Copy)]
pub struct Platform {
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: PLATFORMS_MIN_Y,
            width: PLATFORMS_MAX_WIDTH * 2.0,
            height: PLATFORMS_HEIGHT,
        }
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    tag: Platform,
    name: Name,
}

impl PlatformBundle {
    fn new(color: Color, name: Option<&'static str>, platform: Platform) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(platform.pos_x, platform.pos_y, 10.0),
                    scale: Vec3::new(platform.width, platform.height, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
            tag: platform,
            name: match name {
                Some(name) => Name::new(name),
                None => Name::new("Platform"),
            },
        }
    }
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .init_resource::<WorldPlatforms>()
            .add_systems(
                Update,
                Self::generate_platforms.run_if(in_state(GameState::Game)),
            );

        #[cfg(debug_assertions)]
        {
            app.add_plugins(ResourceInspectorPlugin::<WorldPlatforms>::default());
        }
    }
}

impl PlatformsPlugin {
    pub fn setup(
        mut commands: Commands,
        window: Query<&Window>,
        textures: Res<TextureAssets>,
        layouts: Res<SpriteLayouts>,
    ) {
        let window = window.single();
        let height = window.height();
        let width = window.width();

        let left = width / -2.0;

        commands.spawn(PlatformBundle::new(
            Color::BLACK,
            None,
            Platform {
                pos_x: left,
                pos_y: 0.0,
                width: 200.0,
                height,
            },
        ));

        commands.spawn(PlatformBundle::new(Color::BLACK, None, Platform::default()));

        commands
            .spawn(SpriteSheetBundle {
                texture: textures.cabinet.clone(),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..Default::default()
                },
                atlas: TextureAtlas {
                    layout: layouts.cabinet_layout.clone(),
                    index: 0,
                },
                transform: Transform {
                    translation: Vec3::new(RTE_X, RTE_Y, 20.0),
                    scale: Vec3::new(2.0, 2.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Cabinet"))
            .insert(Cabinet)
            .insert(Sensor)
            .with_children(|commands| {
                commands
                    .spawn(Collider::cuboid(10.0, 5.0))
                    .insert(Sensor)
                    .insert(TransformBundle {
                        local: Transform {
                            translation: Vec3::new(52.3, 10.0, 0.0),
                            scale: Vec3::new(0.4, 3.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Door)
                    .insert(Name::new("Door"));
            });
    }

    fn generate_platforms(mut commands: Commands, mut world_platforms: ResMut<WorldPlatforms>) {
        let mut rng = rand::thread_rng();
        let prev = &world_platforms.last_platform;

        if world_platforms.count < WORLD_MAX_PLATFORMS {
            let width = rng.gen_range(PLATFORMS_MIN_WIDTH..=PLATFORMS_MAX_WIDTH);
            let height = prev.height;

            let next_platform = Platform {
                pos_x: prev.pos_x
                    + (prev.width + width) / 2.0
                    + rng.gen_range(PLATFORMS_MIN_SPACING..=PLATFORMS_MAX_SPACING),
                pos_y: rng.gen_range(PLATFORMS_MIN_Y..=PLATFORMS_MAX_Y),
                width,
                height,
            };
            commands.spawn(PlatformBundle::new(Color::BLACK, None, next_platform));

            world_platforms.count += 1;
            world_platforms.last_platform = next_platform;
        }
    }

    fn scroll_platforms() {}
}
