mod plugins;

use bevy::prelude::*;
use plugins::debug::DebugPlugin;
use plugins::entities::prelude::*;
use plugins::game::prelude::*;

fn main() {
    App::new()
        .add_plugins(GameAssetsPlugin)
        .add_plugins(GameEssentialsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
