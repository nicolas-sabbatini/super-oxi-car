#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
use bevy::{ecs::system::SystemId, prelude::*};
use bevy_turborand::prelude::*;

use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    TILE_SIZE,
};

const SPAWN_DELAY: f32 = 2.5;
const EXPLOTION_DELAY: f32 = 5.0;

#[derive(Component)]
struct BarrelManager {
    dificulty: usize,
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
struct BarrelExplotionAnimation {
    time: Timer,
}

#[derive(Component)]
struct BarrelSprite(usize);

#[derive(Resource)]
struct BarrelAssets(Handle<TextureAtlas>);

#[derive(Resource)]
struct BarrelCount(isize);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_barrel, setup_manager))
            .add_systems(
                Update,
                (manage_barrels, update_barrel_explotion, update_barrel_spawn),
            )
            .insert_resource(BarrelCount(0));
    }
}

fn load_barrel(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_handle = asset_server.load("barrel.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(TILE_SIZE), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(BarrelAssets(texture_atlas_handle));
}

fn setup_manager(world: &mut World) {
    let spawn_barrel = world.register_system(spawn_barrel);
    world.spawn(BarrelManager {
        dificulty: 3,
        spawn_time: Timer::from_seconds(0.5, TimerMode::Repeating),
        spawn_system: spawn_barrel,
    });
}

fn manage_barrels(
    mut barrel_count: ResMut<BarrelCount>,
    mut query: Query<&mut BarrelManager>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let mut barrel_manager = query.single_mut();
    barrel_manager.spawn_time.tick(time.delta());
    while barrel_count.0 < barrel_manager.dificulty as isize {
        commands.run_system(barrel_manager.spawn_system);
        barrel_count.0 += 1;
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
                time: Timer::from_seconds(SPAWN_DELAY, TimerMode::Once),
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
                        texture_atlas: texture_atlas_handle.0.clone(),
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
        });
}

fn get_eased_percent(percent: f32) -> f32 {
    // https://github.com/PistonDevelopers/interpolation/blob/98f3e451b49a3901c9c88581beb80757bd475e07/src/ease.rs#L363
    if percent < 4.0 / 11.0 {
        (121.0 * percent * percent) / 16.0
    } else if percent < 8.0 / 11.0 {
        (363.0 / 40.0 * percent * percent) - (99.0 / 10.0 * percent) + 17.0 / 5.0
    } else if percent < 9.0 / 10.0 {
        (4356.0 / 361.0 * percent * percent) - (35442.0 / 1805.0 * percent) + 16061.0 / 1805.0
    } else {
        (54.0 / 5.0 * percent * percent) - (513.0 / 25.0 * percent) + 268.0 / 25.0
    }
}

fn aply_percent_by_index(index: usize, percent: f32) -> Vec3 {
    let height = f32::max(0.0, WINDOW_HEIGHT - (WINDOW_HEIGHT * percent));
    Vec3::new(0.0, height + index as f32, index as f32)
}

fn update_barrel_spawn(
    mut barrel_query: Query<(&mut BarrelSpawnAnimation, &Children, Entity), With<Barrel>>,
    mut sprites_query: Query<(&mut Transform, &BarrelSprite), Without<Barrel>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut barrel_props, children, entity) in &mut barrel_query {
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            commands.entity(entity).remove::<BarrelSpawnAnimation>();
            commands.entity(entity).insert(BarrelExplotionAnimation {
                time: Timer::from_seconds(EXPLOTION_DELAY, TimerMode::Once),
            });
        }
        let percent = get_eased_percent(barrel_props.time.percent());
        for child in children {
            if let Ok((mut transform, index)) = sprites_query.get_mut(*child) {
                transform.translation = aply_percent_by_index(index.0, percent);
            }
        }
    }
}

fn update_barrel_explotion(
    mut barrel_query: Query<(&mut BarrelExplotionAnimation, Entity), With<Barrel>>,
    time: Res<Time>,
    mut commands: Commands,
    mut barrel_count: ResMut<BarrelCount>,
) {
    for (mut barrel_props, e) in &mut barrel_query {
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            commands.entity(e).despawn_recursive();
            barrel_count.0 -= 1;
        }
    }
}
