use crate::GameAssetsState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::AudioSource;

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

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "embedded://background/background_scaled_0002_buildings_0.png")]
    pub bg_buildings_0: Handle<Image>,
    #[asset(path = "embedded://background/background_scaled_0000_buildings_1.png")]
    pub bg_buildings_1: Handle<Image>,
    #[asset(path = "embedded://background/background_scaled_0003_cloud_0.png")]
    pub bg_cloud_0: Handle<Image>,
    #[asset(path = "embedded://background/background_scaled_0001_cloud_1.png")]
    pub bg_cloud_1: Handle<Image>,

    #[asset(path = "embedded://terrain/right.png")]
    pub building_right: Handle<Image>,
    #[asset(path = "embedded://terrain/left.png")]
    pub building_left: Handle<Image>,
    #[asset(path = "embedded://terrain/middle.png")]
    pub building_middle: Handle<Image>,
    #[asset(path = "embedded://terrain/board.png")]
    pub street_board: Handle<Image>,
    #[asset(path = "embedded://terrain/cabinet.png")]
    pub cabinet: Handle<Image>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "embedded://player.png")]
    pub player: Handle<Image>,
    #[asset(path = "embedded://pts.png")]
    pub pts: Handle<Image>,
    #[asset(path = "embedded://bytes.png")]
    pub byte: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteLayouts {
    #[asset(texture_atlas_layout(tile_size_x = 48.0, tile_size_y = 48.9, columns = 10, rows = 4))]
    pub player_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 110.0, tile_size_y = 65.0, columns = 1, rows = 2))]
    pub cabinet_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 8.0, tile_size_y = 8.0, columns = 9, rows = 9))]
    pub byte_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct FontsAssets {
    #[asset(path = "embedded://fonts/VCR_OSD_MONO.ttf")]
    pub vcr: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "embedded://audio/ambience.ogg")]
    pub ambience: Handle<AudioSource>,
    #[asset(path = "embedded://audio/original.ogg")]
    pub original: Handle<AudioSource>,
    #[asset(path = "embedded://audio/jump.ogg")]
    pub jmup: Handle<AudioSource>,
    #[asset(path = "embedded://audio/death.ogg")]
    pub death: Handle<AudioSource>,
}
