use bevy::utils::default;

use crate::player::{PlayerStats, MIN_SPEED};

const MAX_BUFF_STATS: PlayerStats = PlayerStats {
    max_speed: 10000.0,
    max_health: 10000000,
    bullet_damage: 1000,
    bullet_reload_time: 0.0,
    bullet_heat: 0,
    rocket_reload_time: 0.0,
    dodge_time: 0.5,
    dodge_cooldown: 0.1,
};

const MAX_NERF_STATS: PlayerStats = PlayerStats {
    max_speed: MIN_SPEED,
    max_health: 1,
    bullet_damage: 1,
    bullet_reload_time: 1.0,
    bullet_heat: 100,
    rocket_reload_time: 10.0,
    dodge_time: 0.1,
    dodge_cooldown: 2.5,
};

pub fn buff_stats(_level: usize) -> PlayerStats {
    MAX_BUFF_STATS
}

pub fn nerf_stats(level: usize) -> PlayerStats {
    let min_nerf = PlayerStats::default();
    let max_nerf = MAX_NERF_STATS;
    let percent = (level as f32 / 9.0).clamp(0.0, 1.0);

    PlayerStats {
        max_speed: min_nerf.max_speed + (max_nerf.max_speed - min_nerf.max_speed) * percent,
        // We know that min_nerf has a higher value and because max_nerf > 0 this will not overflow
        max_health: min_nerf.max_health
            - ((min_nerf.max_health - max_nerf.max_health) as f32 * percent) as u32,
        bullet_damage: min_nerf.bullet_damage
            - ((min_nerf.bullet_damage - max_nerf.bullet_damage) as f32 * percent) as u32,
        bullet_reload_time: min_nerf.bullet_reload_time
            + (max_nerf.bullet_reload_time - min_nerf.bullet_reload_time) * percent,
        rocket_reload_time: min_nerf.rocket_reload_time
            + (max_nerf.rocket_reload_time - min_nerf.rocket_reload_time) * percent,
        dodge_time: min_nerf.dodge_time + (max_nerf.dodge_time - min_nerf.dodge_time) * percent,
        ..default()
    }
}
