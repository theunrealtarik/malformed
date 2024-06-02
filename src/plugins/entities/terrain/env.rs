use crate::{GameAssetsState, SpriteLayouts, TextureAssets};
use bevy::prelude::*;
use glib::*;

use super::*;

#[derive(Component)]
pub struct Cabinet;

#[derive(Component)]
pub struct Door;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameAssetsState::Loaded), Self::setup)
            .add_systems(Update, Self::open_door);
    }
}

impl EnvironmentPlugin {
    pub fn setup(
        mut commands: Commands,
        textures: Res<TextureAssets>,
        layouts: Res<SpriteLayouts>,
    ) {
        // roof top exit
        commands
            .spawn(SpriteSheetBundle {
                texture: textures.cabinet.clone(),
                atlas: TextureAtlas {
                    layout: layouts.cabinet_layout.clone(),
                    index: 0,
                },
                transform: Transform {
                    translation: Vec3::new(RTE_X, RTE_Y, 20.0),
                    scale: WORLD_SPRITE_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Cabinet"))
            .insert(Scrollable)
            .insert(Anchor::BottomCenter)
            .insert(Cabinet)
            .with_children(|commands| {
                commands
                    .spawn(Collider::cuboid(10.0, 5.0))
                    .insert(Sensor)
                    .insert(TransformBundle {
                        local: Transform {
                            translation: Vec3::new(20.0, -12.0, 0.0),
                            scale: Vec3::new(0.4, 4.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Door)
                    .insert(Name::new("Door"));
            });

        // street board
        commands
            .spawn(SpriteBundle {
                texture: textures.street_board.clone(),
                transform: Transform {
                    translation: Vec3::new(RTE_X + 200.0, RTE_Y + 40.0, 0.0),
                    scale: WORLD_SPRITE_SCALE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Street Board"))
            .insert(Scrollable);
    }

    fn open_door(
        player: Query<Entity, With<crate::plugins::entities::player::Player>>,
        door: Query<Entity, With<Door>>,
        mut cabinet: Query<(Entity, &mut TextureAtlas), With<Cabinet>>,
        ctx: Res<RapierContext>,
    ) {
        if let (Ok(player), Ok(door), Ok((_, mut atlas))) = (
            player.get_single(),
            door.get_single(),
            cabinet.get_single_mut(),
        ) {
            if ctx.intersection_pair(player, door) == Some(true) {
                atlas.index = 1;
            } else {
                atlas.index = 0;
            }
        }
    }
}
