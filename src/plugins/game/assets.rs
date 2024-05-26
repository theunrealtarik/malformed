use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::plugins::debug::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameAssetsState>().add_loading_state(
            LoadingState::new(GameAssetsState::Pending)
                .continue_to_state(GameAssetsState::Loaded)
                .load_collection::<SpriteLayouts>()
                .load_collection::<TextureAssets>()
                .load_collection::<FontsAssets>()
                .load_collection::<AudioAssets>(),
        );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum GameAssetsState {
    #[default]
    Pending,
    Loaded,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "background/background_scaled_0002_buildings_0.png")]
    pub bg_buildings_0: Handle<Image>,
    #[asset(path = "background/background_scaled_0000_buildings_1.png")]
    pub bg_buildings_1: Handle<Image>,
    #[asset(path = "background/background_scaled_0003_cloud_0.png")]
    pub bg_cloud_0: Handle<Image>,
    #[asset(path = "background/background_scaled_0001_cloud_1.png")]
    pub bg_cloud_1: Handle<Image>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player.png")]
    pub player: Handle<Image>,
    #[asset(path = "cabinet.png")]
    pub cabinet: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteLayouts {
    #[asset(texture_atlas_layout(tile_size_x = 48.0, tile_size_y = 48.9, columns = 10, rows = 4))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 143.0, tile_size_y = 90.0, columns = 1, rows = 2))]
    pub cabinet_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct FontsAssets {
    #[asset(path = "fonts/VCR_OSD_MONO.ttf")]
    pub vcr: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}
