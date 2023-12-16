use bevy::prelude::*;

use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    TILE_SIZE,
};

const CAR_SIZE: f32 = TILE_SIZE * 1.5;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct Movement {
    top_aceleration: f32,
    acceleration: f32,
    acceleration_rate: f32,
    drag: f32,
    angle: f32,
    velocity: Vec3,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_up_player)
            .add_systems(Update, (rotate_player, move_player));
    }
}

fn set_up_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("car.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(TILE_SIZE), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            Movement {
                top_aceleration: 25.0,
                acceleration: 0.0,
                acceleration_rate: 0.1,
                drag: 0.1,
                angle: 0.0,
                velocity: Vec3::new(0.0, 0.0, 0.0),
            },
            Player,
        ))
        .with_children(|player| {
            for i in 0..12 {
                player.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            index: i,
                            flip_x: true,
                            custom_size: Some(Vec2::splat(CAR_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 1.0 * i as f32, i as f32),
                        ..default()
                    },
                    PlayerSprite,
                ));
            }
        });
}

fn rotate_player(
    player_query: Query<(&Children, &Movement), With<Player>>,
    mut sprites_query: Query<&mut Transform, With<PlayerSprite>>,
) {
    for (children, movement) in &player_query {
        let rotation = Quat::from_rotation_z(movement.angle);
        for child in children {
            let mut transform = sprites_query.get_mut(*child).unwrap();
            transform.rotation = rotation;
        }
    }
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>,
) {
    for (mut transform, mut movement) in &mut query {
        let mut dt_angle = 0.0;
        if keys.pressed(KeyCode::A) {
            dt_angle += 0.015;
        }
        if keys.pressed(KeyCode::D) {
            dt_angle -= 0.015;
        }

        movement.acceleration = f32::min(
            movement.top_aceleration,
            movement.acceleration + movement.acceleration_rate,
        );

        if keys.pressed(KeyCode::Space) {
            movement.angle += dt_angle * 1.15;
            let target_velocity = Vec3::new(movement.angle.cos(), movement.angle.sin(), 0.0)
                * (movement.acceleration * 0.80);
            movement.velocity = (target_velocity - movement.velocity) * movement.drag;
        } else {
            movement.angle += dt_angle;
            let target_velocity =
                Vec3::new(movement.angle.cos(), movement.angle.sin(), 0.0) * movement.acceleration;
            movement.velocity = (target_velocity - movement.velocity) * movement.drag;
        }

        transform.translation += movement.velocity;

        // Fix car position if it goes out of screen
        let screen_limit_x = (WINDOW_WIDTH + CAR_SIZE) * 0.5;
        let screen_limit_y = (WINDOW_HEIGHT + CAR_SIZE) * 0.5;
        if transform.translation.x > screen_limit_x {
            transform.translation.x = -screen_limit_x;
        }
        if transform.translation.x < -screen_limit_x {
            transform.translation.x = screen_limit_x;
        }
        if transform.translation.y > screen_limit_y {
            transform.translation.y = -screen_limit_y;
        }
        if transform.translation.y < -screen_limit_y {
            transform.translation.y = screen_limit_y;
        }
    }
}
