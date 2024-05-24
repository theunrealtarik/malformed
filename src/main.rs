mod plugins;

use bevy::prelude::*;
use plugins::game::assets::GameAssetsPlugin;
use plugins::game::menu::GameMenuPlugin;
use plugins::game::GameSetupPlugin;

fn main() {
    App::new()
        .add_plugins(GameAssetsPlugin)
        .add_plugins(GameSetupPlugin)
        .add_plugins(GameMenuPlugin)
        .run();
}
