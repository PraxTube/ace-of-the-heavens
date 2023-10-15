use std::time::Duration;

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_kira_audio::prelude::{AudioSource, *};

use crate::network::ggrs_config::GGRS_FPS;

#[derive(Component, Default, Reflect)]
pub struct RollbackSound {
    /// the actual sound effect to play
    pub clip: Handle<AudioSource>,
    /// when the sound effect should have started playing
    pub start_frame: usize,
    /// differentiates several unique instances of the same sound playing at once. (we'll get back to this)
    pub sub_key: usize,
}

impl RollbackSound {
    pub fn key(&self) -> (Handle<AudioSource>, usize) {
        (self.clip.clone(), self.sub_key)
    }
}

#[derive(Component, Reflect, Default)]
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
}

#[derive(Bundle)]
pub struct RollbackSoundBundle {
    pub sound: RollbackSound,
}

/// The "Actual" state.
///
/// I'm using bevy_kira for sound, but this could probably work similarly with bevy_audio.
#[derive(Resource, Reflect, Default)]
pub struct PlaybackStates {
    pub playing: HashMap<(Handle<AudioSource>, usize), Handle<AudioInstance>>,
}

pub fn sync_rollback_sounds(
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

    // start/update sound effects
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
                let instance_handle = audio.play(rollback_sound.clip.clone()).handle();
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

pub fn remove_finished_sounds(
    frame: Res<FrameCount>,
    query: Query<(Entity, &RollbackSound)>,
    mut commands: Commands,
    audio_sources: Res<Assets<AudioSource>>,
) {
    for (entity, rollback_sound) in query.iter() {
        // perf: cache frames_to_play instead of checking audio_sources every frame?
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

pub fn update_looped_sounds(
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
