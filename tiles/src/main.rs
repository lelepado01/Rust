use amethyst::{
    core::transform::{
        components::Transform, 
        TransformBundle
    }, 
    assets::{AssetStorage, Loader, Handle}, 
    utils::application_root_dir,
    prelude::*, 
    renderer::{
        RenderFlat2D, SpriteSheet, types::DefaultBackend, Texture, RenderingBundle, ImageFormat, 
        Camera, SpriteRender, RenderToWindow, SpriteSheetFormat
    }, 
    input::{InputBundle, StringBindings}, 
};
use rand::Rng;
mod camera;
mod camera_movement;
use camera_movement::CameraMovement;

fn load_textures(world : &mut World) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            
    loader.load(
        "tilemap.png",
        ImageFormat::default(),
        (),
        &texture_storage
    )
}

fn load_spritesheet(world : &mut World, texture_handle : Handle<Texture>) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "tilemap.ron", 
        SpriteSheetFormat(texture_handle), 
        (), 
        &sprite_sheet_storage
    )
}


struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data : StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        camera::initialize_camera(world);
        let texture_handle = load_textures(world);
        let sprite_sheet_handle = load_spritesheet(world, texture_handle);

        let sprite_num = 16; 

        let mut rng = rand::thread_rng();

        for tile_x in 0..sprite_num {
            for tile_y in 0..sprite_num {

                let r : u8 = rng.gen();
                let mut tile_id = 0;
                if r < 50 {
                    tile_id = 1;
                }

                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: tile_id,
                };

                let mut tile_transform = Transform::default();
                tile_transform.set_translation_xyz(tile_x as f32 * 64.0, tile_y as f32 * 64.0, 0.0);

                world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite)
                    .build();
            }
        }

        //
        //let config = &world.read_resource::<PaddleConfig>();
        //println!("{:?}", config.velocity);
    }
}

//
//#[derive(Debug, Deserialize, Serialize)]
//pub struct PaddleConfig {
//    pub height: f32,
//    pub width: f32,
//    pub velocity: f32,
//    pub color: (f32, f32, f32, f32),
//}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let camera_movement_bindings_path = app_root.join("src").join("camera_movement_bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(camera_movement_bindings_path)?;
    //
    //let paddle_config = PaddleConfig::load(&config_dir.join("conf.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(CameraMovement, "camera_movement_system", &["input_system"]);

    let mut game = Application::build(assets_dir, GameplayState)?
        //.with_resource(paddle_config)
        .build(game_data)?;
    game.run();
    
    Ok(())
}