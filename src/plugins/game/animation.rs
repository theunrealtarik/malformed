use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Responsive;

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: &'static [Frame],
}

#[derive(Component)]
pub struct Frame {
    pub duration: Duration,
    pub index: usize,
}

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::animate);
    }
}

impl GameAnimationPlugin {
    fn animate(mut query: Query<(&mut TextureAtlas, &mut Animation)>, time: Res<Time>) {
        for (mut atlas, mut animation) in query.iter_mut() {
            if animation.timer.tick(time.delta()).just_finished() {
                let current_idx = animation
                    .frames
                    .iter()
                    .position(|s| s.index != atlas.index)
                    .unwrap_or(0);

                let next_idx = (current_idx + animation.timer.times_finished_this_tick() as usize)
                    % animation.frames.len();

                atlas.index = animation.frames[next_idx].index;
            }
        }
    }
}
