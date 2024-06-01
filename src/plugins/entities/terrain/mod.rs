use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use glib::*;

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
pub struct Platform {
    pub coords: (f32, f32),
    pub width: f32,
}

#[derive(Component)]
pub struct Scrollable;

// mod obstacles;
mod buildings;
mod env;

// pub use obstacles::*;
pub use buildings::*;
pub use env::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildingsPlugin)
            .add_plugins(EnvironmentPlugin);
    }
}
