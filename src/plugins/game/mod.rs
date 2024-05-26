pub mod animation;
pub mod assets;
pub mod menu;
pub mod movement;
pub mod setup;

pub mod prelude {
    #[derive(bevy::prelude::Component)]
    pub struct Responsive;

    pub use super::animation::*;
    pub use super::assets::*;
    pub use super::menu::*;
    pub use super::movement::*;
    pub use super::setup::*;
}
