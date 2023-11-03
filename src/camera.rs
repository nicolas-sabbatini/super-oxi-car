use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
};

use crate::config::{
    GAME_CAMERA_CLEAR_COLOR, GAME_CAMERA_NAME, GAME_CAMERA_TARGET_NAME, WINDOW_CAMERA_CLEAR_COLOR,
    WINDOW_CAMERA_NAME, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const BGRA_PIXEL_SIZE: usize = 4;
const WHITE_BGRA: [u8; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize * BGRA_PIXEL_SIZE] =
    [255; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize * BGRA_PIXEL_SIZE];

#[derive(Debug, Component)]
pub struct GameCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Set up windows camera
    let mut windows_camera = Camera2dBundle::default();
    windows_camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: WINDOW_WIDTH,
        min_height: WINDOW_HEIGHT,
    };
    // Set up clear color
    windows_camera.camera_2d.clear_color = ClearColorConfig::Custom(WINDOW_CAMERA_CLEAR_COLOR);
    // Set up camera order to be the last
    windows_camera.camera.order = 2;
    // Spawn windows camera
    commands
        .spawn(windows_camera)
        .insert(Name::new(WINDOW_CAMERA_NAME))
        // Only draw layer 1
        .insert(RenderLayers::layer(1));

    // Set up letter boxing
    // Create render target texture
    let render_target_size = Extent3d {
        width: WINDOW_WIDTH as u32,
        height: WINDOW_HEIGHT as u32,
        ..default()
    };
    // Create render target image in NOT wasm targets
    #[cfg(not(target_arch = "wasm32"))]
    let mut render_target_image = Image::new_fill(
        render_target_size,
        TextureDimension::D2,
        &WHITE_BGRA,
        TextureFormat::Bgra8UnormSrgb,
    );
    // Create render target image in wasm targets
    #[cfg(target_arch = "wasm32")]
    let mut render_target_image = Image::new_fill(
        render_target_size,
        TextureDimension::D2,
        &WHITE_BGRA,
        TextureFormat::Rgba8UnormSrgb,
    );
    // By default an image can't be used as a render target so we need to setup the render target falg
    render_target_image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    // Add the render target to the image assets
    let render_target_handle = images.add(render_target_image);
    // Spawn render target on the world
    commands
        .spawn(SpriteBundle {
            texture: render_target_handle.clone(),
            ..Default::default()
        })
        .insert(Name::new(GAME_CAMERA_TARGET_NAME))
        // Only the windows camera can see the render target
        .insert(RenderLayers::layer(1));

    // Set up game camera
    let mut game_camera = Camera2dBundle::default();
    // Set up the render target created previously as target
    game_camera.camera.target = RenderTarget::Image(render_target_handle);
    game_camera.camera_2d.clear_color = ClearColorConfig::Custom(GAME_CAMERA_CLEAR_COLOR);
    // Give the game camere the highest order
    game_camera.camera.order = 1;
    // Spawn game camera
    commands
        .spawn(game_camera)
        .insert(Name::new(GAME_CAMERA_NAME))
        .insert(GameCamera);
}
