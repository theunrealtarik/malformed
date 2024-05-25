mod background;
mod camera;
mod player;

pub mod prelude {
    pub use super::background::BackgroundPlugin;
    pub use super::camera::CameraPlugin;
    pub use super::player::PlayerPlugin;
}
