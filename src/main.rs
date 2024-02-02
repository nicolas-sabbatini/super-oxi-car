use bevy::{prelude::*, window::WindowResolution};
use bevy_turborand::prelude::RngPlugin;
use config::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

mod barrel;
mod camera;
mod config;
mod particle;
mod player;
mod ui;

const TILE_SIZE: f32 = 32.0;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    title: WINDOW_TITLE.to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        RngPlugin::default(),
    ))
    .insert_resource(Msaa::Off);

    app.add_plugins((
        camera::Plug,
        player::Plug,
        ui::Plug,
        barrel::Plug,
        particle::Plug,
    ));

    app.run();
}
