use amethyst::{
    core::transform::{TransformBundle, Transform},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,ImageFormat, Texture, SpriteSheet, SpriteRender, SpriteSheetFormat, Camera
    },
    assets::{Loader, AssetStorage, Handle},
    utils::application_root_dir,
};

mod player;
use player::Player;
mod movement;
use movement::MovementSystem;

struct MyState{
    pub sprite_handle : Option<Handle<SpriteSheet>>
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_handle.replace(load_texture(world));
        initiliza_player(world, self.sprite_handle.clone().unwrap());
        initialize_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(MovementSystem, "movement_system", &[]);

    let mut game = Application::new(assets_dir, MyState{sprite_handle:None}, game_data)?;
    game.run();

    Ok(())
}

fn initiliza_player(world : &mut World, sprite_handle : Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0,250.0,0.0);

    let player = Player {
        position : transform
    };

    let sprite_renderer = SpriteRender {
        sprite_sheet : sprite_handle, 
        sprite_number : 0
    };

    world
        .create_entity()
        .with(sprite_renderer)
        .with(player.position.clone())
        .with(player)
        .build();
}

fn load_texture(world : &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "player.png", 
            ImageFormat::default(), 
            (), 
            &texture_storage
        )
    };

    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "player.ron", 
        SpriteSheetFormat(texture_handle), 
        (), 
        &sprite_sheet_storage
    )
}

fn initialize_camera(world : &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(250.0, 250.0))
        .with(transform)
        .build();
}
