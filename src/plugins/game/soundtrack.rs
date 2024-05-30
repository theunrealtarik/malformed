use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{plugins::entities::player::Being, AudioAssets, GameState};

#[derive(Resource)]
struct InstanceHandle(Handle<AudioInstance>);

pub struct GameSoundTrack;

impl Plugin for GameSoundTrack {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Resumed), Self::play_background_audio)
            .add_systems(
                Update,
                Self::control_background_audio.run_if(resource_exists::<InstanceHandle>),
            );
    }
}

impl GameSoundTrack {
    fn play_background_audio(
        mut commands: Commands,
        audio_assets: Res<AudioAssets>,
        audio: Res<Audio>,
    ) {
        let handle = audio
            .play(audio_assets.original.clone())
            .fade_in(AudioTween::new(
                Duration::from_secs(5),
                AudioEasing::OutPowi(2),
            ))
            .loop_from(38.392)
            .loop_until(44.781)
            .with_volume(0.3)
            .handle();
        commands.insert_resource(InstanceHandle(handle));
    }

    fn control_background_audio(
        background_audio: Res<InstanceHandle>,
        mut being_events: EventReader<StateTransitionEvent<Being>>,
        mut audio_instances: ResMut<Assets<AudioInstance>>,
        audio_assets: Res<AudioAssets>,
        audio: Res<Audio>,
    ) {
        if let Some(instance) = audio_instances.get_mut(&background_audio.0) {
            for ev in being_events.read() {
                match instance.state() {
                    PlaybackState::Playing { position: _ } => {
                        if ev.before == Being::Alive && ev.after == Being::Dead {
                            instance.pause(AudioTween::new(
                                Duration::from_secs(1),
                                AudioEasing::OutPowi(1),
                            ));
                            audio.play(audio_assets.death.clone()).with_volume(0.5);
                        }
                    }
                    PlaybackState::Paused { position: _ } => {
                        if ev.before == Being::Dead && ev.after == Being::Alive {
                            instance.resume(AudioTween::new(
                                Duration::from_secs(1),
                                AudioEasing::OutPowi(1),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
