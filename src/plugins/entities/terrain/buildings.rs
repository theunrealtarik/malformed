use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use glib::WORLD_SPRITE_SCALE;
use rand::Rng;

use super::*;
use crate::*;

use crate::plugins::debug::*;

#[derive(Default)]
pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(Update, Self::despawn)
            .add_systems(
                Update,
                (Self::scroll, Self::generate)
                    .run_if(in_state(GameState::Resumed))
                    .run_if(in_state(Being::Alive)),
            )
            .register_type::<Building>()
            .register_type::<Platform>();

        app.add_plugins(EntityInspector::<Platform>::default());
    }
}

impl BuildingsPlugin {
    pub fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
        Building::spawn(
            &mut commands,
            &textures,
            8,
            8.0 * BUILDING_WIDTH,
            PLATFORMS_MIN_Y,
            1.0,
        )
        .insert(PreventByte);
    }

    pub fn generate(
        mut commands: Commands,
        mut platforms: Query<(&Platform, &Transform)>,
        textures: Res<TextureAssets>,
        velocity: Query<&AuxiliaryVelocity, With<Player>>,
    ) {
        let Ok(velocity) = velocity.get_single() else {
            return;
        };

        let mut rng = rand::thread_rng();
        let mut platforms = platforms.iter_mut().collect::<Vec<_>>();

        platforms.sort_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap());

        let (prev, prev_trans) = platforms.last().unwrap();

        if platforms.len() < WORLD_MAX_PLATFORMS as usize {
            let growth = |dir: f32| 1.0 + dir * velocity.value.x / PLAYER_MAX_VELOCITY_X;

            let segments: usize = rng.gen_range(0..=10);
            let width = (segments + 2) as f32 * BUILDING_WIDTH * WORLD_SPRITE_SCALE.x;
            let spacing = (PLATFORMS_MAX_SPACING - PLATFORMS_MIN_SPACING)
                + rng.gen_range(PLATFORMS_MIN_SPACING..=PLATFORMS_MAX_SPACING) * growth(1.0);

            let x = prev_trans.translation.x + (prev.width + width) / 2.0 + spacing;
            let y = rng.gen_range(PLATFORMS_MIN_Y..=PLATFORMS_MAX_Y);
            Building::spawn(&mut commands, &textures, segments, x, y, 10.0);
        }
    }

    fn despawn(mut commands: Commands, platforms: Query<(Entity, &Transform), With<Scrollable>>) {
        if platforms.is_empty() {
            return;
        }

        for (entity, transform) in platforms.iter() {
            if transform.translation.x <= 10f32.powi(3) * -8.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    fn scroll(
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

#[derive(Default, Debug, Component, Reflect)]
pub struct Building {
    reps: usize,
    width: f32,
    pos_x: f32,
    pos_y: f32,
}

impl Building {
    pub fn spawn<'a>(
        commands: &'a mut Commands,
        textures: &'a Res<TextureAssets>,
        reps: usize,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
    ) -> bevy::ecs::system::EntityCommands<'a> {
        let reps = reps + 2;
        let width = reps as f32 * BUILDING_WIDTH;

        let mut entity_commands = commands.spawn(Name::new("Building"));
        entity_commands
            .insert(Visibility::Visible)
            .insert(InheritedVisibility::default())
            .insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos_x, pos_y, pos_z),
                    scale: WORLD_SPRITE_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Platform {
                coords: (pos_x, pos_y),
                width: width * WORLD_SPRITE_SCALE.x,
            })
            .insert(Scrollable)
            .insert(Collider::cuboid(width / 2.0, BUILDING_HEIGHT / 2.0))
            .insert(Anchor::TopCenter)
            .with_children(|parent| {
                let half_ext = BUILDING_WIDTH / 2.0;
                let mut prev_x = width / -2.0 + half_ext * -1.0;

                for i in 0..reps {
                    let x = prev_x + BUILDING_WIDTH;
                    prev_x = x;

                    parent
                        .spawn(Name::new("Building Segment"))
                        .insert(SpriteBundle {
                            texture: if i == 0 {
                                textures.building_left.clone()
                            } else if i == reps - 1 {
                                textures.building_right.clone()
                            } else {
                                textures.building_middle.clone()
                            },
                            transform: Transform::from_xyz(x, 0.0, 0.0),
                            ..Default::default()
                        })
                        .insert(Anchor::TopCenter);
                }
            });

        entity_commands
    }
}
