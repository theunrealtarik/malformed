pub mod animation;
pub mod assets;
pub mod ground;
pub mod menu;
pub mod restart;
pub mod setup;
pub mod states;

pub mod prelude {
    #[derive(bevy::prelude::Component, Default)]
    pub struct Responsive;

    pub use super::animation::*;
    pub use super::assets::*;
    pub use super::ground::*;
    pub use super::menu::*;
    pub use super::restart::*;
    pub use super::setup::*;
    pub use super::states::*;
}
