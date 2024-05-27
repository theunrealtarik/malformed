pub mod animation;
pub mod assets;
pub mod ground;
pub mod menu;
pub mod setup;

pub mod prelude {
    #[derive(bevy::prelude::Component)]
    pub struct Responsive;

    pub use super::animation::*;
    pub use super::assets::*;
    pub use super::ground::*;
    pub use super::menu::*;
    pub use super::setup::*;
}
