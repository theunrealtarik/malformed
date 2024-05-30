use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// mod obstacles;
mod platforms;

// pub use obstacles::*;
pub use platforms::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlatformsPlugin)
            .add_systems(Update, Self::open_door);
    }
}

impl TerrainPlugin {
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
