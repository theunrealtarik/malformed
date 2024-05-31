pub mod bsod;
use bevy::prelude::*;

pub const APP_WINDOW_NAME: &str = "Malformed";
pub const APP_WINDOW_MIN_WIDTH: f32 = 600.0;
pub const APP_WINDOW_MIN_HEIGHT: f32 = 400.0;
pub const APP_WINDOW_DESIRED_WITH: f32 = 1920.0;
pub const APP_WINDOW_DESIRED_HEIGHT: f32 = 1080.0;

pub const WORLD_BACKGROUND_COLOR: Color = Color::rgb(164.0 / 255.0, 206.0 / 255.0, 215.0 / 255.0);

pub mod utils {
    use crate::{APP_WINDOW_DESIRED_HEIGHT, APP_WINDOW_DESIRED_WITH};
    use bevy::prelude::*;

    pub fn rescale(transform: &mut Transform, width: f32, height: f32) {
        transform.scale *= Vec3::new(
            width / APP_WINDOW_DESIRED_WITH,
            height / APP_WINDOW_DESIRED_HEIGHT,
            0.0,
        );
    }

    pub mod easings {
        pub fn expo(scalar: f32, rate: f32, intv: f32) -> f32 {
            scalar * (1.0 - rate.powf(intv))
        }
    }
}
