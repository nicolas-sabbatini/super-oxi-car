use bevy::{prelude::*, text::BreakLineOn};

use crate::config::{P8_WINE, WINDOW_HEIGHT, WINDOW_WIDTH};

const SCORE_HEIGHT: f32 = 64.0;
const SCORE_OFFSET: f32 = WINDOW_WIDTH * -0.4;

#[derive(Component)]
struct Score;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("ArchivoBlack-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: P8_WINE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new("Score: ", text_style.clone()),
                    TextSection::new("000000", text_style),
                ],
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::AnyCharacter,
            },
            transform: Transform::from_translation(Vec3::new(
                SCORE_OFFSET,
                (WINDOW_HEIGHT - SCORE_HEIGHT) * 0.5,
                99.,
            )),
            ..default()
        },
        Score,
    ));
}

fn update_ui(mut query: Query<&mut Text, With<Score>>) {
    let mut text = query.single_mut();
    let score = text.sections[1].value.parse::<usize>().unwrap();
    text.sections[1].value = format!("{:0>6}", score + 1);
}
