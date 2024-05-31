use std::time::Duration;

pub(super) const PLAYER_SCALE_X: f32 = 2.0;
pub(super) const PLAYER_SCALE_Y: f32 = 2.0;
pub(super) const PLAYER_MASS: f32 = 100.0;

pub(super) const PLAYER_COLLIDER_WIDTH: f32 = 24.0;
pub(super) const PLAYER_COLLIDER_HEIGHT: f32 = 34.0;

pub(super) const PLAYER_RISE_GRAVITY: f32 = 1.0;
pub(super) const PLAYER_FALL_GRAVITY: f32 = 1.8;

pub(super) const PLAYER_COYOTE_JUMP_TIME: f32 = 0.35;
pub(super) const PLAYER_JUMP_BUFFERING_TIME: f32 = 0.3;
pub(crate) const PLAYER_JUMP_HEIGHT: f32 = 200.0;

pub(super) const PLAYER_WALKING_TIMER: Duration = Duration::from_secs(6);

pub(crate) const PLAYER_RESPAWN_VELOCITY: f32 =
    (PLAYER_INIT_VELOCITY_X + PLAYER_VELOCITY_BUMP) * 2.0;
pub(crate) const PLAYER_MAX_VELOCITY_X: f32 = 1000.0;
pub(crate) const PLAYER_VELOCITY_BUMP: f32 = 150.0;
pub(crate) const PLAYER_INIT_VELOCITY_X: f32 = 80.0;

pub const PLAYER_JUMP_WINDOW: f32 = 0.3;

pub(crate) const PLAYER_MAX_STAMINA: f32 = 100.0;
pub(crate) const PLAYER_STAMINA_RECOVERY_RATE: f32 = 20.0;

pub(super) static DIALOG_LINES: [(&str, Duration); 4] = [
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
        Duration::from_secs(3),
    ),
];
