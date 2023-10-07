use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_hanabi::prelude::*;

use crate::debug::DebugTransform;
use crate::map::map::outside_of_borders;
use crate::map::obstacle::{collision, Obstacle};
use crate::network::GgrsConfig;
use crate::Rematch;
use crate::RollbackState;
use crate::{input, GameAssets};

use crate::player::health::spawn_health_bar;
use crate::player::reloading::spawn_reload_bars;
use crate::player::shooting::Bullet;
use crate::player::shooting::BulletTimer;

// Movement
pub const MAX_SPEED: f32 = 400.0 / 60.0;
pub const MIN_SPEED: f32 = 200.0 / 60.0;
pub const DELTA_SPEED: f32 = 75.0 / 60.0 / 100.0;
pub const DELTA_STEERING: f32 = 3.5 / 60.0;
// Collision
pub const PLAYER_RADIUS: f32 = 24.0;
// Health
pub const MAX_HEALTH: u32 = 2000;
// Spawning
const PLAYER_SCALE: f32 = 1.75;
const DISTANCE_FROM_SPAWN: f32 = 800.0;
// Color
pub const P1_COLOR: Color = Color::rgb(
    0xDF as f32 / 255.0,
    0x71 as f32 / 255.0,
    0x26 as f32 / 255.0,
);
pub const P2_COLOR: Color = Color::rgb(
    0x20 as f32 / 255.0,
    0x8E as f32 / 255.0,
    0xD9 as f32 / 255.0,
);

#[derive(Component, Reflect, Default)]
#[reflect(Hash)]
pub struct Player {
    pub handle: usize,

    pub current_speed: f32,
    pub health: u32,
    pub heat: u32,
    pub overheated: bool,
}

impl Player {
    fn new(handle: usize) -> Player {
        Player {
            handle,
            current_speed: MIN_SPEED,
            health: MAX_HEALTH,
            heat: 0,
            overheated: false,
        }
    }

    pub fn speed_ratio(&self) -> u32 {
        ((self.current_speed - MIN_SPEED).max(0.0) / (MAX_SPEED - MIN_SPEED).max(0.0) * 100.0)
            as u32
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current_speed.to_bits().hash(state);
    }
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

pub fn destroy_players(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player, &Transform), Without<Bullet>>,
    obstacles: Query<&Obstacle, (Without<Player>, Without<Bullet>)>,
    mut next_state: ResMut<NextState<RollbackState>>,
) {
    for (player_entity, mut player, transform) in &mut players {
        if player.health <= 0 || outside_of_borders(transform.translation) {
            player.health = 0;
            commands.entity(player_entity).despawn_recursive();
            continue;
        }

        for obstacle in &obstacles {
            if collision(obstacle, transform.translation, PLAYER_RADIUS) {
                player.health = 0;
                commands.entity(player_entity).despawn_recursive();
            }
        }
    }

    if players.iter().count() <= 1 {
        next_state.set(RollbackState::RoundEnd);
    }
}

fn spawn_trail_effect(
    commands: &mut Commands,
    spawn_position: Vec3,
    spawn_rotation: Quat,
    texture: Handle<Image>,
    effects: &mut ResMut<Assets<EffectAsset>>,
) -> Entity {
    // Create a color gradient for the particles
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.00).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(0.0).expr(),
    };

    let spawner = Spawner::rate(100.0.into());
    let effect = effects.add(
        EffectAsset::new(100, spawner, writer.finish())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ParticleTextureModifier { texture })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(10.0)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect),
            transform: Transform::from_translation(spawn_position).with_rotation(spawn_rotation),
            ..default()
        })
        .id()
}

fn spawn_player(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    handle: usize,
    spawn_position: Vec3,
    spawn_rotation: Quat,
) -> Entity {
    let transform = Transform::from_scale(Vec3::splat(PLAYER_SCALE))
        .with_translation(spawn_position)
        .with_rotation(spawn_rotation);
    commands
        .spawn((
            Player::new(handle),
            BulletTimer::default(),
            DebugTransform::new(&transform),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
        ))
        .add_rollback()
        .id()
}

pub fn spawn_players(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<GameAssets>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle = assets.player_1.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 0;
    let position = Vec3::new(-DISTANCE_FROM_SPAWN, 0.0, 0.0);
    let rotation = Quat::from_rotation_z(0.0);
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        position,
        rotation,
    );
    let texture = assets.score_full.clone();
    let trail_left = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, 15.0, -1.0),
        rotation,
        texture.clone(),
        &mut effects,
    );
    let trail_right = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, -15.0, -1.0),
        rotation,
        texture.clone(),
        &mut effects,
    );
    commands
        .entity(player)
        .push_children(&[trail_left, trail_right]);
    spawn_health_bar(&mut commands, handle, position);
    spawn_reload_bars(&mut commands, handle, position);

    let texture_handle = assets.player_2.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let handle: usize = 1;
    let position = Vec3::new(DISTANCE_FROM_SPAWN, 0.0, 0.0);
    let rotation = Quat::from_rotation_z(std::f32::consts::PI);
    let player = spawn_player(
        &mut commands,
        texture_atlas_handle,
        handle,
        position,
        rotation,
    );
    let trail_left = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, 15.0, -1.0),
        rotation,
        texture.clone(),
        &mut effects,
    );
    let trail_right = spawn_trail_effect(
        &mut commands,
        Vec3::new(0.0, -15.0, -1.0),
        rotation,
        texture.clone(),
        &mut effects,
    );
    commands
        .entity(player)
        .push_children(&[trail_left, trail_right]);
    spawn_health_bar(&mut commands, handle, position);
    spawn_reload_bars(&mut commands, handle, position);
}

pub fn check_rematch_state(mut rematch: ResMut<Rematch>, inputs: Res<PlayerInputs<GgrsConfig>>) {
    if input::rematch(inputs[0].0) {
        rematch.0 = true;
    }
    if input::rematch(inputs[1].0) {
        rematch.1 = true;
    }
}
