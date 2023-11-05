use bevy::prelude::*;

use crate::config::{P8_CREAM, WINDOW_HEIGHT, WINDOW_WIDTH};

pub const TOP_BAR_HEIGHT: f32 = 64.0;
const TOP_BAR_WIDTH: f32 = WINDOW_WIDTH;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: P8_CREAM,
            custom_size: Some(Vec2::new(TOP_BAR_WIDTH, TOP_BAR_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0,
            (WINDOW_HEIGHT - TOP_BAR_HEIGHT) * 0.5,
            99.,
        )),
        ..default()
    });
}
