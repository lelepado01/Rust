use amethyst::{
    prelude::*, // containing ? 
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow}, // plugins to render
        types::DefaultBackend, // type of rendering bundle
        RenderingBundle, // rendering container
    },
    utils::application_root_dir, // function -> project root directory
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{UiBundle, RenderUi}
};
use crate::pong::Pong;

mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default()); // exception handling

    let app_root = application_root_dir()?; // project root
    let display_config_path = app_root.join("config").join("display.ron"); // display config

    let bindings_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default() // create game data 
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new() // with rendeing bundle
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?  // adding towindow rendering, with config taken from config file
                        .with_clear([0.0, 0.0, 0.0, 1.0]), // when clearing the screen default = (0,0,0)
                )
                .with_plugin(
                    RenderFlat2D::default() // renderer for 2D apps (?)
                )
                .with_plugin(
                    RenderUi::default()
                )
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(systems::BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets"); // asset folder directory
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?; // create new application, with game data

    game.run(); // start game loop

    Ok(())
}