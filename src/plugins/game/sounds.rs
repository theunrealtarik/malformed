use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{plugins::entities::player::Being, AudioAssets, GameAssetsState, GameState};

#[derive(Resource)]
struct InstanceHandle(Handle<AudioInstance>);

const LOOP_POSITION: (f64, f64) = (38.392, 44.781);

pub struct GameSoundTrack;

impl Plugin for GameSoundTrack {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Resumed),
            Self::play_background_soundtrack,
        )
        .add_systems(
            Update,
            Self::control_background_audio.run_if(resource_exists::<InstanceHandle>),
        )
        .add_systems(
            OnEnter(GameAssetsState::Loaded),
            Self::play_background_ambience,
        );
    }
}

impl GameSoundTrack {
    fn play_background_ambience(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
        audio
            .play(audio_assets.ambience.clone())
            .with_volume(0.3)
            .looped();
    }

    fn play_background_soundtrack(
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
            .loop_from(LOOP_POSITION.0)
            .loop_until(LOOP_POSITION.1)
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
                                Duration::from_secs_f32(0.0),
                                AudioEasing::OutPowi(0),
                            ));
                            audio.play(audio_assets.death.clone()).with_volume(0.5);
                        }
                    }
                    PlaybackState::Paused { position: _ } => {
                        if ev.before == Being::Dead && ev.after == Being::Alive {
                            instance.seek_to(38.392);
                            instance.resume(AudioTween::new(
                                Duration::from_secs_f32(0.2),
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
