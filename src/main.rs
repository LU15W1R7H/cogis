pub mod brain;
pub mod bundle;
pub mod corgi;
pub mod genes;
pub mod neural_network;
pub mod systems;
pub mod universe;
pub mod util;

extern crate nalgebra as na;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::bundle::CorgiBundle;
use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use crate::universe::Universe;

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            app_root.join("config/input_controller.ron")
        } else {
            app_root.join("config/input.ron")
        }
    };

    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(CorgiBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Universe::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
