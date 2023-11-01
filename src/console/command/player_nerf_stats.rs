use crate::player::{PlayerStats, MIN_SPEED};

const MAX_NERF_STATS: PlayerStats = PlayerStats {
    max_speed: MIN_SPEED,
    max_health: 1,
    bullet_damage: 1,
    rocket_reload_time: 10.0,
};

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
        rocket_reload_time: min_nerf.rocket_reload_time
            + (max_nerf.rocket_reload_time - min_nerf.rocket_reload_time) * percent,
    }
}
