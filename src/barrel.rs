use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component)]
struct Barrel;

#[derive(Component)]
struct BarrelSprite;

#[derive(Component)]
struct BarrelProperties {
    angle: f32,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_up_barrel)
            .add_systems(Update, (update_barrel, reotate_barrel));
    }
}

fn set_up_barrel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("barrel.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(TILE_SIZE), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            BarrelProperties { angle: 0.0 },
            Barrel,
        ))
        .with_children(|barrel| {
            for i in 0..12 {
                barrel.spawn((
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
                    BarrelSprite,
                ));
            }
        });
}

fn update_barrel(
    barrel_query: Query<(&Children, &BarrelProperties), With<Barrel>>,
    mut sprites_query: Query<&mut Transform, With<BarrelSprite>>,
) {
    for (children, movement) in &barrel_query {
        let rotation = Quat::from_rotation_z(movement.angle);
        for child in children {
            let mut transform = sprites_query.get_mut(*child).unwrap();
            transform.rotation = rotation;
        }
    }
}

fn reotate_barrel(mut query: Query<&mut BarrelProperties, With<Barrel>>) {
    for mut movement in &mut query {
        movement.angle += 0.015;
    }
}
