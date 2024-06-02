pub mod animation;
pub mod assets;
pub mod dialog;
pub mod ground;
pub mod menu;
pub mod restart;
pub mod rpc;
pub mod setup;
pub mod sounds;
pub mod states;

pub mod prelude {
    use bevy::prelude::*;

    #[derive(Component, Default)]
    pub struct Responsive;

    pub use super::animation::*;
    pub use super::assets::*;
    pub use super::dialog::*;
    pub use super::ground::*;
    pub use super::menu::*;
    pub use super::restart::*;
    pub use super::rpc::*;
    pub use super::setup::*;
    pub use super::sounds::*;
    pub use super::states::*;
}
