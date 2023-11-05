use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use config::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

mod camera;
mod config;
mod player;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0),
                    title: WINDOW_TITLE.to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(Msaa::Off);

    app.add_plugins((CameraPlugin, player::Plug, ui::Plug));

    app.run();
}
