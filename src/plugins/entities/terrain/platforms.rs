use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;

use crate::*;
use plugins::debug::*;

use self::plugins::entities::player::*;

const PLATFORMS_MAX_Y: f32 = PLAYER_JUMP_HEIGHT * 0.5 + PLATFORMS_MIN_Y;
const PLATFORMS_MIN_Y: f32 = -640.0;
const PLATFORMS_MAX_SPACING: f32 = 400.0;
const PLATFORMS_MIN_SPACING: f32 = 100.0;
const PLATFORMS_MAX_WIDTH: f32 = 1000.0;
const PLATFORMS_MIN_WIDTH: f32 = 500.0;
const PLATFORMS_HEIGHT: f32 = 1000.0;

const WORLD_MAX_PLATFORMS: u8 = 10;

const RTE_X: f32 = 0.0;
const RTE_Y: f32 = PLATFORMS_MIN_Y + PLATFORMS_HEIGHT / 2.0;

#[derive(Component)]
pub struct Scrollable;

#[derive(Component)]
pub struct Cabinet;

#[derive(Component)]
pub struct Door;

#[derive(Component, Reflect, Clone, Copy, Debug)]
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
            width: PLATFORMS_MAX_WIDTH * 10.0,
            height: PLATFORMS_HEIGHT,
        }
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    data: Platform,
    scrollable: Scrollable,
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
            data: platform,
            scrollable: Scrollable,
            name: match name {
                Some(name) => Name::new(name),
                None => Name::new("Platform"),
            },
        }
    }
}

#[derive(Default)]
pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(Update, Self::despawn_platforms)
            .add_systems(
                Update,
                (Self::scroll_platforms, Self::generate_platforms)
                    .run_if(in_state(GameState::Resumed))
                    .run_if(in_state(Being::Alive)),
            )
            .add_plugins(EntityInspector::<Platform>::default());
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

        commands.spawn(PlatformBundle::new(
            Color::BLACK,
            Some("Initial Platform"),
            Platform::default(),
        ));

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
            .insert(Scrollable)
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

    fn generate_platforms(
        mut commands: Commands,
        mut platforms: Query<(&Platform, &Transform)>,
        velocity: Query<&AuxiliaryVelocity, With<Player>>,
    ) {
        let Ok(velocity) = velocity.get_single() else {
            return;
        };

        let mut rng = rand::thread_rng();
        let mut platforms = platforms.iter_mut().collect::<Vec<_>>();

        platforms.sort_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap());

        let (prev, prev_trans) = match platforms.last() {
            Some((&platform, &transform)) => (platform, transform),
            None => (Platform::default(), Transform::default()),
        };

        if platforms.len() < WORLD_MAX_PLATFORMS as usize {
            let growth = 1.0 + velocity.value.x / PLAYER_MAX_VELOCITY_X;

            let width = (PLATFORMS_MAX_WIDTH - PLATFORMS_MIN_WIDTH)
                + rng.gen_range(PLATFORMS_MIN_WIDTH..=PLATFORMS_MAX_WIDTH) * growth
                + PLATFORMS_MIN_WIDTH;

            let spacing = (PLATFORMS_MAX_SPACING - PLATFORMS_MIN_SPACING)
                + rng.gen_range(PLATFORMS_MIN_SPACING..=PLATFORMS_MAX_SPACING) * growth
                + PLATFORMS_MIN_SPACING;

            let height = prev.height;
            let next_platform = Platform {
                pos_x: prev_trans.translation.x + (prev.width + width) / 2.0 + spacing,
                pos_y: rng.gen_range(PLATFORMS_MIN_Y..=PLATFORMS_MAX_Y),
                width,
                height,
            };

            commands.spawn(PlatformBundle::new(Color::BLACK, None, next_platform));
        }
    }

    fn despawn_platforms(
        mut commands: Commands,
        platforms: Query<(Entity, &Transform), With<Scrollable>>,
    ) {
        if platforms.is_empty() {
            return;
        }

        for (entity, transform) in platforms.iter() {
            if transform.translation.x <= PLATFORMS_MAX_WIDTH * -8.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn scroll_platforms(
        mut platforms: Query<&mut Transform, With<Scrollable>>,
        time: Res<Time>,
        velocity: Query<&AuxiliaryVelocity, With<Player>>,
    ) {
        if platforms.is_empty() {
            return;
        }

        let velocity = velocity.single();
        for mut platform in platforms.iter_mut() {
            platform.translation.x += -2.0 * velocity.value.x * time.delta_seconds();
        }
    }
}
