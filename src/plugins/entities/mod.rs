pub mod background;
pub mod camera;
pub mod platforms;
pub mod player;

pub mod prelude {
    pub use super::background::BackgroundPlugin;
    pub use super::camera::GameCameraPlugin;
    pub use super::platforms::PlatformsPlugin;
    pub use super::player::PlayerPlugin;
}
