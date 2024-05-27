mod plugins;

use bevy::prelude::*;
use plugins::debug::DebugPlugin;
use plugins::entities::prelude::*;
use plugins::game::prelude::*;

fn main() {
    App::new()
        .add_plugins(GameAssetsPlugin)
        .add_plugins(GameEssentialsPlugin)
        .add_plugins(GameAnimationPlugin)
        .add_plugins(GameCameraPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
