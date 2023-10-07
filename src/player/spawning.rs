use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use bevy_hanabi::prelude::*;

use super::player::{Player, PLAYER_RADIUS};

use crate::debug::DebugTransform;
use crate::map::map::outside_of_borders;
use crate::map::obstacle::{collision, Obstacle};
use crate::GameAssets;
use crate::RollbackState;

use crate::player::health::spawn_health_bar;
use crate::player::reloading::spawn_reload_bars;
use crate::player::shooting::Bullet;
use crate::player::shooting::BulletTimer;

const PLAYER_SCALE: f32 = 1.75;
const DISTANCE_FROM_SPAWN: f32 = 800.0;

pub fn despawn_players(
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
