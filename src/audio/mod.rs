mod bgm;

pub use bgm::BgmStage;

use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_ggrs::GgrsSchedule;
use bevy_kira_audio::prelude::{AudioPlugin, AudioSource, *};

use crate::network::ggrs_config::GGRS_FPS;
use crate::{GameState, RollbackState};

const MAIN_VOLUME: f64 = 0.35;

#[derive(Component, Reflect)]
pub struct RollbackSound {
    /// the actual sound effect to play
    pub clip: Handle<AudioSource>,
    /// when the sound effect should have started playing
    pub start_frame: usize,
    /// differentiates several unique instances of the same sound playing at once.
    pub sub_key: usize,
    pub volume: f64,
    pub playback_rate: f64,
}

impl RollbackSound {
    pub fn key(&self) -> (Handle<AudioSource>, usize) {
        (self.clip.clone(), self.sub_key)
    }
}

impl Default for RollbackSound {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            start_frame: 0,
            sub_key: 0,
            volume: 1.0,
            playback_rate: 1.0,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FadedLoopSound {
    /// The actual sound playing, if any
    pub audio_instance: Option<Handle<AudioInstance>>,
    /// The sound to play
    pub clip: Handle<AudioSource>,
    /// number of seconds to fade in
    pub fade_in: f32,
    /// number of seconds to fade out
    pub fade_out: f32,
    /// whether the sound effect should be playing or not
    pub should_play: bool,
    pub despawn_on_silence: bool,
    pub volume: f64,
}

impl Default for FadedLoopSound {
    fn default() -> Self {
        Self {
            audio_instance: None,
            clip: Handle::default(),
            fade_in: 0.0,
            fade_out: 0.0,
            should_play: true,
            despawn_on_silence: false,
            volume: 1.0,
        }
    }
}

/// The "Actual" state.
///
/// I'm using bevy_kira for sound, but this could probably work similarly with bevy_audio.
#[derive(Resource, Reflect, Default)]
pub struct PlaybackStates {
    pub playing: HashMap<(Handle<AudioSource>, usize), Handle<AudioInstance>>,
}

fn sync_rollback_sounds(
    mut current_state: ResMut<PlaybackStates>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    desired_query: Query<&RollbackSound>,
    audio: Res<Audio>,
    frame: Res<FrameCount>,
) {
    // remove any finished sound effects
    current_state.playing.retain(|_, handle| {
        !matches!(
            audio_instances.state(handle),
            PlaybackState::Stopped | PlaybackState::Stopping { .. }
        )
    });

    let mut live = HashSet::new();

    for rollback_sound in desired_query.iter() {
        let key = rollback_sound.key();
        if current_state.playing.contains_key(&key) {
            // already playing
            // todo: compare frames and seek if time critical
        } else {
            let frames_late = frame.0 as usize - rollback_sound.start_frame;
            const MAX_SOUND_DELAY: usize = 10;
            // ignore any sound effects that are *really* late
            // todo: make configurable
            if frames_late <= MAX_SOUND_DELAY {
                if frames_late > 0 {
                    // todo: seek if time critical
                    info!(
                        "playing sound effect {} frames late",
                        frame.0 as usize - rollback_sound.start_frame
                    );
                }
                let instance_handle = audio
                    .play(rollback_sound.clip.clone())
                    .with_volume(rollback_sound.volume * MAIN_VOLUME)
                    .with_playback_rate(rollback_sound.playback_rate)
                    .handle();
                current_state
                    .playing
                    .insert(key.to_owned(), instance_handle);
            }
        }

        // we keep track of `RollbackSound`s still existing,
        // so we can remove any sound effects not present later
        live.insert(rollback_sound.key().to_owned());
    }

    // stop interrupted sound effects
    for (_, instance_handle) in current_state
        .playing
        .extract_if(|key, _| !live.contains(key))
    {
        if let Some(instance) = audio_instances.get_mut(&instance_handle) {
            // todo: add config to use linear tweening, stop or keep playing as appropriate
            // instance.stop(default()); // immediate
            instance.stop(AudioTween::linear(Duration::from_millis(100)));
        } else {
            error!("Audio instance not found");
        }
    }
}

fn remove_finished_sounds(
    mut commands: Commands,
    frame: Res<FrameCount>,
    audio_sources: Res<Assets<AudioSource>>,
    query: Query<(Entity, &RollbackSound)>,
) {
    for (entity, rollback_sound) in query.iter() {
        if let Some(audio_source) = audio_sources.get(&rollback_sound.clip) {
            let frames_played = frame.0 as usize - rollback_sound.start_frame;
            let seconds_to_play = audio_source.sound.duration().as_secs_f64();
            let frames_to_play = (seconds_to_play * GGRS_FPS as f64) as usize;

            if frames_played >= frames_to_play {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn update_looped_sounds(
    mut sounds: Query<&mut FadedLoopSound>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    audio: Res<Audio>,
) {
    for mut sound in sounds.iter_mut() {
        if sound.should_play {
            if sound.audio_instance.is_none() {
                sound.audio_instance = Some(
                    audio
                        .play(sound.clip.clone())
                        .looped()
                        .linear_fade_in(Duration::from_secs_f32(sound.fade_in))
                        .with_volume(sound.volume)
                        .handle(),
                );
            }
        } else if let Some(instance_handle) = sound.audio_instance.take() {
            if let Some(instance) = audio_instances.get_mut(&instance_handle) {
                instance.stop(AudioTween::linear(Duration::from_secs_f32(sound.fade_out)));
            }
        };
    }
}

fn remove_looped_sounds(mut commands: Commands, query: Query<(Entity, &FadedLoopSound)>) {
    for (entity, sound) in &query {
        if !sound.despawn_on_silence {
            continue;
        }

        if !sound.should_play && sound.audio_instance.is_none() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<PlaybackStates>()
            .add_systems(OnEnter(GameState::MainMenu), bgm::fade_out_all_bgm)
            .add_systems(Update, (sync_rollback_sounds, update_looped_sounds))
            .add_systems(OnEnter(RollbackState::RoundStart), bgm::check_bgm_stage)
            .add_systems(
                OnEnter(RollbackState::GameOver),
                bgm::fade_out_game_over_bgm,
            )
            .add_systems(
                GgrsSchedule,
                (remove_finished_sounds, remove_looped_sounds)
                    .chain()
                    .after(apply_state_transition::<RollbackState>),
            );
    }
}
