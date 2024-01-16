use bevy::{core::FrameCount, prelude::*};
use bevy_ggrs::AddRollbackCommandExtension;

use crate::audio::RollbackSound;
use crate::camera::CameraShake;
use crate::input::GamepadRumble;
use crate::player::{health::PlayerTookDamage, LocalPlayerHandle};
use crate::GameAssets;

#[derive(Component)]
pub struct DamageEffectSpawner;

pub fn spawn_damage_effect_sound(
    mut commands: Commands,
    assets: Res<GameAssets>,
    frame: Res<FrameCount>,
    mut ev_player_took_damage: EventReader<PlayerTookDamage>,
) {
    for _ in ev_player_took_damage.iter() {
        commands
            .spawn(RollbackSound {
                clip: assets.damage_sound.clone(),
                start_frame: frame.0 as usize,
                sub_key: frame.0 as usize,
                ..default()
            })
            .add_rollback();
    }
}

pub fn add_camera_shake_damage(
    mut camera_shake: ResMut<CameraShake>,
    mut ev_player_took_damage: EventReader<PlayerTookDamage>,
    local_handle: Res<LocalPlayerHandle>,
) {
    for ev in ev_player_took_damage.iter() {
        // Local player took damage, time to shake it
        if ev.handle == local_handle.0 {
            camera_shake.add_trauma(0.15);
        }
    }
}

pub fn add_gamepad_rumble(
    mut gamepad_rumble: ResMut<GamepadRumble>,
    mut ev_player_took_damage: EventReader<PlayerTookDamage>,
    local_handle: Res<LocalPlayerHandle>,
) {
    for ev in ev_player_took_damage.iter() {
        // Local player took damage, time to shake it
        if ev.handle == local_handle.0 {
            gamepad_rumble.add_rumble(0.15, 0.2);
        }
    }
}
