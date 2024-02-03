#![allow(clippy::needless_pass_by_value, clippy::cast_precision_loss)]
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};
use interpolation::Ease;

use crate::TILE_SIZE;

const PARTICLE_LIFE: f32 = 0.5;

#[derive(Resource)]
struct ParticleAssets {
    texture: Handle<Image>,
}

#[derive(Event)]
pub struct SpawnEvent(pub Vec3);

#[derive(Component)]
struct Particle {
    timer: Timer,
    velosity: Vec3,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_systems(Startup, load_particles)
            .add_systems(Update, (spawn_particle, update_particle));
    }
}

fn load_particles(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(ParticleAssets {
        texture: asset_server.load("fire.png"),
    });
}

fn spawn_particle(
    mut commands: Commands,
    particle_assets: Res<ParticleAssets>,
    mut spawn_event: EventReader<SpawnEvent>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for pos in spawn_event.read() {
        for _i in 0..global_rng.usize(20..30) {
            let pos_offset = Vec3::new(
                (global_rng.f32() - 0.5) * TILE_SIZE,
                (global_rng.f32() - 0.5) * TILE_SIZE,
                (global_rng.f32() - 0.5) * TILE_SIZE + 30.0,
            );
            let x_velosity = (global_rng.f32() - 0.5) * 300.0;
            let y_velosity = global_rng.f32() * 100.0 + 150.0;
            let velosity = Vec3::new(x_velosity, y_velosity, 0.0);
            commands.spawn((
                SpriteBundle {
                    texture: particle_assets.texture.clone(),
                    transform: Transform::from_translation(pos.0 + pos_offset),
                    ..Default::default()
                },
                Particle {
                    timer: Timer::from_seconds(PARTICLE_LIFE, TimerMode::Once),
                    velosity,
                },
            ));
        }
    }
    spawn_event.clear();
}

fn update_particle(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(&mut Transform, &mut Particle, &mut Sprite, Entity)>,
) {
    for (mut pos, mut properties, mut sprite, entity) in &mut particle_query {
        properties.timer.tick(time.delta());
        if properties.timer.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            let percent = properties.timer.percent().exponential_in();
            let new_color = Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0 - percent,
            };
            sprite.color = new_color;
            pos.translation += properties.velosity * time.delta_seconds();
        }
    }
}
