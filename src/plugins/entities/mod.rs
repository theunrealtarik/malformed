pub mod background;
pub mod camera;
pub mod player;
pub mod terrain;

pub mod prelude {
    pub use super::background::BackgroundPlugin;
    pub use super::camera::GameCameraPlugin;
    pub use super::player::PlayerPlugin;
    pub use super::terrain::TerrainPlugin;
}
