#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
use bevy::prelude::*;
use bevy_turborand::prelude::*;

use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    TILE_SIZE,
};

#[derive(Component)]
struct Barrel;

#[derive(Component)]
struct BarrelSprite;

#[derive(Component)]
struct BarrelProperties {
    angle: f32,
    time: Timer,
}

#[derive(Component)]
struct BarrelManager {
    dificulty: usize,
    rng: RngComponent,
}

#[derive(Resource)]
struct BarrelAssets(Handle<TextureAtlas>);

#[derive(Resource)]
struct BarrelCount(isize);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_barrel, setup_manager))
            .add_systems(Update, (manage_barrels, update_barrel))
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

fn setup_manager(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.spawn(BarrelManager {
        dificulty: 10,
        rng: RngComponent::from(&mut global_rng),
    });
}

fn manage_barrels(
    mut commands: Commands,
    texture_atlas_handle: Res<BarrelAssets>,
    mut barrel_count: ResMut<BarrelCount>,
    mut query: Query<&mut BarrelManager>,
) {
    let mut barrel_manager = query.single_mut();
    let x_limit = (WINDOW_WIDTH * 0.5) as i32;
    let y_limit = (WINDOW_HEIGHT * 0.5) as i32;

    while barrel_count.0 < barrel_manager.dificulty as isize {
        let x = barrel_manager.rng.i32(-x_limit..=x_limit) as f32;
        let y = barrel_manager.rng.i32(-y_limit..=y_limit) as f32;
        let angle = barrel_manager
            .rng
            .sample::<f32>(&[
                0.0,
                0.5 * std::f32::consts::PI,
                std::f32::consts::PI,
                1.5 * std::f32::consts::PI,
            ])
            .unwrap();
        dbg!(x, y, angle);
        spawn_barrel(&mut commands, &texture_atlas_handle, x, y, *angle);
        barrel_count.0 += 1;
    }
}

fn spawn_barrel(
    commands: &mut Commands,
    texture_atlas_handle: &Res<BarrelAssets>,
    x: f32,
    y: f32,
    angle: f32,
) {
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
            BarrelProperties {
                angle,
                time: Timer::from_seconds(1.0, TimerMode::Once),
            },
            Barrel,
        ))
        .with_children(|barrel| {
            for i in 0..12 {
                barrel.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.0.clone(),
                        sprite: TextureAtlasSprite {
                            index: i,
                            flip_x: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 1.0 * i as f32, i as f32),
                        ..default()
                    },
                    BarrelSprite,
                ));
            }
        });
}

fn update_barrel(
    mut barrel_query: Query<(&Children, &mut BarrelProperties, Entity), With<Barrel>>,
    mut sprites_query: Query<&mut Transform, With<BarrelSprite>>,
    time: Res<Time>,
    mut commands: Commands,
    mut barrel_count: ResMut<BarrelCount>,
) {
    for (children, mut barrel_props, e) in &mut barrel_query {
        let rotation = Quat::from_rotation_z(barrel_props.angle);
        for child in children {
            let mut transform = sprites_query.get_mut(*child).unwrap();
            transform.rotation = rotation;
        }
        barrel_props.time.tick(time.delta());
        if barrel_props.time.finished() {
            commands.entity(e).despawn_recursive();
            barrel_count.0 -= 1;
        }
    }
}
