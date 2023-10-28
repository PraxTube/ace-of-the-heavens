use crate::player::PlayerStats;

const MAX_NERF_STATS: PlayerStats = PlayerStats { max_speed: 0.0 };

pub fn nerf_stats(level: usize) -> PlayerStats {
    let min_nerf = PlayerStats::default();
    let max_nerf = MAX_NERF_STATS;
    let percent = (level as f32 / 9.0).clamp(0.0, 1.0);

    PlayerStats {
        max_speed: min_nerf.max_speed + (max_nerf.max_speed - min_nerf.max_speed) * percent,
    }
}
