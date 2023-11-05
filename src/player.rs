use bevy::prelude::*;

use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    ui::TOP_BAR_HEIGHT,
};

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct Movement {
    top_aceleration: f32,
    acceleration: f32,
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
                top_aceleration: 17.0,
                acceleration: 0.0,
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
        if keys.pressed(KeyCode::A) {
            movement.angle += 0.015;
        }
        if keys.pressed(KeyCode::D) {
            movement.angle -= 0.015;
        }

        movement.acceleration = f32::min(movement.top_aceleration, movement.acceleration + 0.1);
        let target_velocity =
            Vec3::new(movement.angle.cos(), movement.angle.sin(), 0.0) * movement.acceleration;
        movement.velocity = (target_velocity - movement.velocity) * movement.drag;
        transform.translation += movement.velocity;
        let limit_x = (WINDOW_WIDTH + TILE_SIZE) * 0.5;
        let limit_y = (WINDOW_HEIGHT + TILE_SIZE) * 0.5;
        if transform.translation.x > limit_x {
            transform.translation.x = -limit_x;
        }
        if transform.translation.x < -limit_x {
            transform.translation.x = limit_x;
        }
        if transform.translation.y > limit_y - TOP_BAR_HEIGHT {
            transform.translation.y = -limit_y;
        }
        if transform.translation.y < -limit_y {
            transform.translation.y = limit_y - TOP_BAR_HEIGHT;
        }
    }
}
