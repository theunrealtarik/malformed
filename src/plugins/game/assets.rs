use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameAssetsState>().add_loading_state(
            LoadingState::new(GameAssetsState::Pending)
                .continue_to_state(GameAssetsState::Loaded)
                .load_collection::<PlayerTextureAssets>()
                .load_collection::<UiTextureAssets>()
                .load_collection::<FontsAssets>()
                .load_collection::<AudioAssets>(),
        );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameAssetsState {
    #[default]
    Pending,
    Loaded,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerTextureAssets {}

#[derive(AssetCollection, Resource)]
pub struct UiTextureAssets {
    #[asset(path = "ui/pts.png")]
    pub pts: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontsAssets {
    #[asset(path = "fonts/VCR_OSD_MONO.ttf")]
    pub vcr: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}
