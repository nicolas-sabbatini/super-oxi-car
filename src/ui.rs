use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

use crate::config::{P8_CREAM, WINDOW_HEIGHT, WINDOW_WIDTH};

pub const TOP_BAR_HEIGHT: f32 = 64.0;
const TOP_BAR_WIDTH: f32 = WINDOW_WIDTH;

const BOX_SIZE: Vec2 = Vec2::new(TOP_BAR_WIDTH, TOP_BAR_HEIGHT);
const TEXT_SIZE: Vec2 = Vec2::new(TOP_BAR_WIDTH * 0.5, TOP_BAR_HEIGHT);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: P8_CREAM,
            custom_size: Some(BOX_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0,
            (WINDOW_HEIGHT - TOP_BAR_HEIGHT) * 0.5,
            98.0,
        )),
        ..default()
    });
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new("Player 1 Score: 0", TextStyle::default())],
            alignment: TextAlignment::Left,
            linebreak_behavior: BreakLineOn::AnyCharacter,
        },
        text_2d_bounds: Text2dBounds { size: TEXT_SIZE },
        transform: Transform::from_translation(Vec3::new(
            TOP_BAR_WIDTH * -0.25,
            (WINDOW_HEIGHT - TOP_BAR_HEIGHT) * 0.5,
            99.,
        )),
        ..default()
    });
}
