#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::type_complexity
)]
use bevy::{
    ecs::system::SystemId,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_turborand::prelude::*;
use interpolation::Ease;

use crate::{
    config::{P8_GREY, P8_RED, WINDOW_HEIGHT, WINDOW_WIDTH},
    TILE_SIZE,
};

const SPAWN_MIN_DELAY: f32 = 0.4;
const SPAWN_ANIMATION_DURATION: f32 = 2.5;
const EXPLOSION_ANIMATION_DURATION: f32 = 5.0;

#[derive(Component)]
struct BarrelManager {
    dificulty: f32,
    spawn_time: Timer,
    spawn_system: SystemId,
}

#[derive(Component)]
struct Barrel;

#[derive(Component)]
struct BarrelSpawnAnimation {
    time: Timer,
}

#[derive(Component)]
struct BarrelAlive {
    time: Timer,
}

#[derive(Component)]
struct BarrelExplosionAnimation {
    time: Timer,
}

#[derive(Component)]
struct BarrelSprite(usize);

#[derive(Component)]
struct BarrelShadow;

#[derive(Resource)]
struct BarrelAssets {
    sprite_texture: Handle<TextureAtlas>,
    shadow_mesh: Mesh2dHandle,
    shadow_material: Handle<ColorMaterial>,
    explosion_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct BarrelCount(f32);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_barrel, setup_manager))
            .add_systems(
                Update,
                (
                    manage_barrels,
                    update_barrel_explosion,
                    update_barrel_alive,
                    update_barrel_spawn_animation,
                ),
            )
            .insert_resource(BarrelCount(0.0));
    }
}

fn load_barrel(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("barrel.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(TILE_SIZE), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(BarrelAssets {
        sprite_texture: texture_atlas_handle,
        shadow_mesh: meshes
            .add(shape::Circle::new(TILE_SIZE * 0.5).into())
            .into(),
        shadow_material: materials.add(P8_GREY.into()),
        explosion_material: materials.add(P8_RED.into()),
    });
}

fn setup_manager(world: &mut World) {
    let spawn_barrel = world.register_system(spawn_barrel);
    world.spawn(BarrelManager {
        dificulty: 4.0,
        spawn_time: Timer::from_seconds(SPAWN_MIN_DELAY, TimerMode::Once),
        spawn_system: spawn_barrel,
    });
}

fn manage_barrels(
    mut barrel_count: ResMut<BarrelCount>,
    mut query: Query<&mut BarrelManager>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    let mut barrel_manager = query.single_mut();
    barrel_manager.spawn_time.tick(time.delta());
    if barrel_manager.spawn_time.finished() {
        barrel_manager.spawn_time =
            Timer::from_seconds(SPAWN_MIN_DELAY + global_rng.f32(), TimerMode::Once);
        if barrel_count.0 < barrel_manager.dificulty {
            commands.run_system(barrel_manager.spawn_system);
            barrel_count.0 += 1.0;
        }
    }
}

fn spawn_barrel(
    mut commands: Commands,
    texture_atlas_handle: Res<BarrelAssets>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let x_limit = ((WINDOW_WIDTH - TILE_SIZE) * 0.5) as i32;
    let y_limit = ((WINDOW_HEIGHT - TILE_SIZE) * 0.5) as i32;

    let x = global_rng.i32(-x_limit..=x_limit) as f32;
    let y = global_rng.i32(-y_limit..=y_limit) as f32;
    let angle = global_rng
        .sample::<f32>(&[
            0.0,
            0.5 * std::f32::consts::PI,
            std::f32::consts::PI,
            1.5 * std::f32::consts::PI,
        ])
        .unwrap();

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
            BarrelSpawnAnimation {
                time: Timer::from_seconds(SPAWN_ANIMATION_DURATION, TimerMode::Once),
            },
            Barrel,
        ))
        .with_children(|barrel| {
            let rotation = Quat::from_rotation_z(*angle);
            for i in 0..12 {
                let mut transform = Transform::from_xyz(0.0, WINDOW_HEIGHT + i as f32, i as f32);
                transform.rotation = rotation;
                barrel.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.sprite_texture.clone(),
                        sprite: TextureAtlasSprite {
                            index: i,
                            flip_x: true,
                            ..default()
                        },
                        transform,
                        ..default()
                    },
                    BarrelSprite(i),
                ));
            }
            barrel.spawn((
                MaterialMesh2dBundle {
                    mesh: texture_atlas_handle.shadow_mesh.clone(),
                    material: texture_atlas_handle.shadow_material.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.0)),
                    ..default()
                },
                BarrelShadow,
            ));
        });
}

fn update_barrel_spawn_animation(
    mut barrel_query: Query<
        (&mut BarrelSpawnAnimation, &Children, Entity),
        (With<Barrel>, Without<BarrelSprite>, Without<BarrelShadow>),
    >,
    mut sprites_query: Query<
        (&mut Transform, &BarrelSprite),
        (Without<Barrel>, Without<BarrelShadow>),
    >,
    mut shadow_query: Query<
        &mut Transform,
        (With<BarrelShadow>, Without<Barrel>, Without<BarrelSprite>),
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut barrel_count: ResMut<BarrelCount>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (mut barrel_props, children, entity) in &mut barrel_query {
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            // Remove 1/4 of a barrel
            barrel_count.0 -= 0.25;
            commands.entity(entity).remove::<BarrelSpawnAnimation>();
            commands.entity(entity).insert(BarrelAlive {
                time: Timer::from_seconds(global_rng.f32() * 10.0, TimerMode::Once),
            });
        }
        let percent = barrel_props.time.percent().bounce_out();
        let current_height = f32::max(0.0, WINDOW_HEIGHT - (WINDOW_HEIGHT * percent));
        for child in children {
            if let Ok((mut transform, index)) = sprites_query.get_mut(*child) {
                transform.translation =
                    Vec3::new(0.0, current_height + index.0 as f32, index.0 as f32);
            } else if let Ok(mut transform) = shadow_query.get_mut(*child) {
                transform.scale = Vec3::new(percent, percent, 0.0);
            }
        }
    }
}

fn update_barrel_alive(
    mut barrel_query: Query<(&mut BarrelAlive, Entity), With<Barrel>>,
    time: Res<Time>,
    mut commands: Commands,
    mut barrel_count: ResMut<BarrelCount>,
) {
    for (mut barrel_props, e) in &mut barrel_query {
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            // Remove 1/4 of a barrel
            barrel_count.0 -= 0.25;
            commands.entity(e).remove::<BarrelAlive>();
            commands.entity(e).insert(BarrelExplosionAnimation {
                time: Timer::from_seconds(EXPLOSION_ANIMATION_DURATION, TimerMode::Once),
            });
        }
    }
}

fn update_barrel_explosion(
    mut barrel_query: Query<
        (&mut BarrelExplosionAnimation, &Children, Entity),
        (With<Barrel>, Without<BarrelSprite>, Without<BarrelShadow>),
    >,
    mut sprites_query: Query<
        (&mut Transform, &BarrelSprite),
        (Without<Barrel>, Without<BarrelShadow>),
    >,
    mut shadow_query: Query<
        (&mut Transform, &mut Handle<ColorMaterial>),
        (With<BarrelShadow>, Without<Barrel>, Without<BarrelSprite>),
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut barrel_count: ResMut<BarrelCount>,
    texture_atlas_handle: Res<BarrelAssets>,
) {
    for (mut barrel_props, children, entity) in &mut barrel_query {
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            commands.entity(entity).despawn_recursive();
            // Remove 2/4 of a barrel
            barrel_count.0 -= 0.50;
        }
        for child in children {
            if let Ok((mut transform, index)) = sprites_query.get_mut(*child) {
                let percent = barrel_props.time.percent() * 0.25;
                let sin = ((barrel_props.time.percent() + index.0 as f32 * 0.11) * 40.0).sin()
                    * 0.15
                    + 0.15;
                transform.scale = Vec3::new(1.0 + percent + sin, 1.0 + percent + sin, 1.0);
            }
            if let Ok((mut transform, mut color)) = shadow_query.get_mut(*child) {
                transform.scale = Vec3::new(
                    1.0 + (barrel_props.time.percent() * 1.5),
                    1.0 + (barrel_props.time.percent() * 1.5),
                    1.0,
                );
                *color = texture_atlas_handle.explosion_material.clone();
            }
        }
    }
}
