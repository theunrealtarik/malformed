pub mod bsod;
use std::time::Duration;

use bevy::prelude::*;

pub static DISCORD_APP_ID: &str = "1246393574065442837";
pub static DISCORD_STATE: &str = "Malformed";
pub static DISCORD_LARGE_IMAGE: &str = "default";

pub const APP_WINDOW_NAME: &str = "Malformed";
pub const APP_WINDOW_MIN_WIDTH: f32 = 600.0;
pub const APP_WINDOW_MIN_HEIGHT: f32 = 400.0;
pub const APP_WINDOW_DESIRED_WITH: f32 = 1920.0;
pub const APP_WINDOW_DESIRED_HEIGHT: f32 = 1080.0;

pub const WORLD_BACKGROUND_COLOR: Color = Color::rgb(164.0 / 255.0, 206.0 / 255.0, 215.0 / 255.0);
pub const WORLD_SPRITE_SCALE: Vec3 = Vec3::new(2.0, 2.0, 1.0);

// player
pub const PLAYER_SCALE_X: f32 = WORLD_SPRITE_SCALE.x;
pub const PLAYER_SCALE_Y: f32 = WORLD_SPRITE_SCALE.y;
pub const PLAYER_MASS: f32 = 100.0;
pub const PLAYER_COLLIDER_WIDTH: f32 = 24.0;
pub const PLAYER_COLLIDER_HEIGHT: f32 = 34.0;
pub const PLAYER_RISE_GRAVITY: f32 = 1.0;
pub const PLAYER_FALL_GRAVITY: f32 = 1.8;
pub const PLAYER_COYOTE_JUMP_TIME: f32 = 0.35;
pub const PLAYER_JUMP_BUFFERING_TIME: f32 = 0.3;
pub const PLAYER_JUMP_HEIGHT: f32 = 200.0;
pub const PLAYER_WALKING_TIMER: Duration = Duration::from_secs(6);
pub const PLAYER_RESPAWN_VELOCITY: f32 = (PLAYER_INIT_VELOCITY_X + PLAYER_VELOCITY_BUMP) * 2.0;
pub const PLAYER_MAX_VELOCITY_X: f32 = 1200.0;
pub const PLAYER_VELOCITY_BUMP: f32 = 150.0;
pub const PLAYER_INIT_VELOCITY_X: f32 = 80.0;
pub const PLAYER_JUMP_WINDOW: f32 = 0.3;
pub const PLAYER_MAX_STAMINA: f32 = 30.0;
pub const PLAYER_STAMINA_RECOVERY_RATE: f32 = 20.0;

pub static DIALOG_LINES: [(&str, Duration); 4] = [
    ("that cursed komboter again... F$@!", Duration::from_secs(3)),
    (
        "i can't believe it can't even handle booting up",
        Duration::from_secs(3),
    ),
    (
        "i gottta hurry and get those parts asap",
        Duration::from_secs(3),
    ),
    (
        "i don't want another blue screen ...",
        Duration::from_secs(2),
    ),
];

// camera
pub const CAMERA_STARTING_POSITIION: Vec2 = Vec2::new(0.0, 128.0);
pub const CAMERA_PLAYER_OFFSET: Vec2 = Vec2::new(512.0, 256.0);

// terrain
pub const PLATFORMS_MAX_Y: f32 = PLAYER_JUMP_HEIGHT * 0.4 + PLATFORMS_MIN_Y;
pub const PLATFORMS_MIN_Y: f32 = -512.0;
pub const PLATFORMS_MAX_SPACING: f32 = 500.0;
pub const PLATFORMS_MIN_SPACING: f32 = 100.0;
pub const WORLD_MAX_PLATFORMS: u8 = 10;

pub const BUILDING_WIDTH: f32 = 220.0;
pub const BUILDING_HEIGHT: f32 = 260.0;

pub const RTE_X: f32 = 0.0;
pub const RTE_Y: f32 = PLATFORMS_MIN_Y + WORLD_SPRITE_SCALE.y * 130.0 + 65.0;

// background
pub const BACKGROUND_IMAGE_WIDTH: f32 = 4608.0;
pub const BACKGROUND_LAYER_Y: f32 = 512.0;
pub const BACKGROUND_LAYER_SACLE: f32 = 2.0;
// const BACKGROUND_IMAGE_HEIGHT: f32 = 512.0;

//
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
