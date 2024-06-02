#![allow(clippy::type_complexity)]

use super::terrain::{BuildingsPlugin, Platform};
use crate::{Animation, AuxiliaryVelocity, GameAssetsState, Player, SpriteLayouts, TextureAssets};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use rand::prelude::*;

use glib::{
    BUILDING_HEIGHT, PLAYER_MAX_VELOCITY_X, PLAYER_MEMORY_SHARDS_SPAWN_RATE_MODIFIER,
    WORLD_SPRITE_SCALE,
};

#[derive(Default, Debug, Component)]
pub struct Byte {
    translation: Vec3,
    direction: f32,
}

impl Byte {
    fn new(x: f32, y: f32, direction: f32) -> Self {
        Self {
            translation: Vec3::new(x, y, 0.0),
            direction,
        }
    }
}

#[derive(Default, Debug, Component)]
pub struct PreventByte;

pub struct BytesPlugin;
impl Plugin for BytesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::spawn, Self::float)
                .chain()
                .after(BuildingsPlugin::generate)
                .run_if(in_state(GameAssetsState::Loaded)),
        );
    }
}

const MAX_FLOATING_Y: f32 = BUILDING_HEIGHT / 2.0 + 15.0;
const MIN_FLOATING_Y: f32 = BUILDING_HEIGHT / 2.0 + 10.0;

impl BytesPlugin {
    pub fn spawn(
        mut commands: Commands,
        platform_query: Query<(Entity, &Platform), Without<PreventByte>>,
        player_query: Query<&AuxiliaryVelocity, With<Player>>,
        textures: Res<TextureAssets>,
        layouts: Res<SpriteLayouts>,
    ) {
        if platform_query.is_empty() {
            return;
        }

        let Ok(velocity) = player_query.get_single() else {
            return;
        };

        for (entity, platform) in platform_query.iter() {
            let mut rng = rand::thread_rng();

            if rng.gen_bool(
                (PLAYER_MEMORY_SHARDS_SPAWN_RATE_MODIFIER
                    * (1.0 - velocity.value.x / PLAYER_MAX_VELOCITY_X))
                    .into(),
            ) {
                let mid = (platform.width - 300.0) / (2.0 * WORLD_SPRITE_SCALE.x);

                let x = rng.gen_range(-1.0 * mid..mid);
                let y = (MAX_FLOATING_Y - MIN_FLOATING_Y) / 2.0 + MIN_FLOATING_Y;

                commands
                    .entity(entity)
                    .insert(PreventByte)
                    .with_children(|parent| {
                        parent
                            .spawn(SpriteSheetBundle {
                                texture: textures.byte.clone(),
                                atlas: TextureAtlas {
                                    layout: layouts.byte_layout.clone(),
                                    index: 0,
                                },
                                transform: Transform {
                                    translation: Vec3::new(x, y, 10.0),
                                    scale: WORLD_SPRITE_SCALE,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Name::new("Byte"))
                            .insert(Byte::new(x, y, 1.0))
                            .insert(Collider::cuboid(4.0, 4.0))
                            .insert(Sensor)
                            .insert(Animation::auto(
                                Duration::from_millis(30),
                                TimerMode::Repeating,
                                72,
                            ));
                    });
            }
        }
    }

    pub fn float(mut query: Query<(&mut Transform, &mut Byte)>, time: Res<Time>) {
        if query.is_empty() {
            return;
        }

        for (mut transform, mut byte) in query.iter_mut() {
            byte.translation.y += byte.direction * (time.delta_seconds().sin() + 1.0) / 14.0;

            if byte.translation.y >= MAX_FLOATING_Y {
                byte.direction = -1.0;
            } else if byte.translation.y <= MIN_FLOATING_Y {
                byte.direction = 1.0;
            }

            transform.translation.y = byte.translation.y;
        }
    }
}
