
extern crate tiled;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiled::parse;

use amethyst::core::transform::components::Transform;
use amethyst::core::transform::TransformBundle;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::utils::application_root_dir;
use amethyst::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::ecs::prelude::{Entity};
use amethyst::renderer::{RenderFlat2D, Sprite, SpriteSheet, types::DefaultBackend,
                        sprite::TextureCoordinates, Texture, RenderingBundle, ImageFormat, 
                        Camera, camera::Projection, SpriteRender, RenderToWindow};


pub fn initialize_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(10.0);

    world
        .create_entity()
        .with(
            Camera::from(
                Projection::orthographic(
                    0.0, width, 0.0, height, 0.0, 0.0
        )))
        .with(camera_transform)
        .build()
}


struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data : StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // LOAD TILESET IMAGE
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "assets/terrainTiles_default.png",
                ImageFormat::default(),
                (),
                &texture_storage
            )
        };
        // END

        // We need the camera to actually see anything
        initialize_camera(world);

        // Get the game window screen height
        let screen_height = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.height()
        };

        let file = File::open(&Path::new("assets/tiled_base64_zlib.tmx")).unwrap();
        let reader = BufReader::new(file);
        let map = parse(reader).unwrap();

        if let Some(map_tileset) = map.get_tileset_by_gid(1) {
            let tile_width = map_tileset.tile_width as i32;
            let tile_height = map_tileset.tile_height as i32;
            let tileset_width = &map_tileset.images[0].width;
            let tileset_height = &map_tileset.images[0].height;

            let tileset_sprite_columns = tileset_width / tile_width as i32;
            let tileset_sprite_offset_colums = 1.0 / tileset_sprite_columns as f32;

            let tileset_sprite_rows = tileset_height / tile_height as i32;
            let tileset_sprite_offset_rows = 1.0 / tileset_sprite_rows as f32;
            
            let mut tile_sprites: Vec<Sprite> = Vec::new();

            for x in (0..tileset_sprite_rows).rev() {
                for y in 0..tileset_sprite_columns {

                    let tex_coords = TextureCoordinates {
                        left: y as f32 * tileset_sprite_offset_colums,
                        right: (y + 1) as f32 * tileset_sprite_offset_colums,
                        bottom: x as f32 * tileset_sprite_offset_rows,
                        top: (x + 1) as f32 * tileset_sprite_offset_rows
                    };

                    let sprite = Sprite {
                        width: tile_width as f32,
                        height: tile_height as f32,
                        offsets: [0.0, 0.0],
                        tex_coords
                    };

                    tile_sprites.push(sprite);
                }
            }

            let sprite_sheet = SpriteSheet {
                texture: texture_handle,
                sprites: tile_sprites
            };

            // Insert the sprite sheet, which consists of all the tile sprites,
            // into world resources for later use
            let sprite_sheet_handle = {
                let loader = world.read_resource::<Loader>();
                let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

                loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
            };

            // Now that all the tile sprites/textures are loaded in
            // we can start drawing the tiles for our viewing pleasure
            let layer: &tiled::Layer = &map.layers[0];

            for (y, row) in layer.tiles.iter().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    // Do nothing with empty tiles
                    if tile == 0 {
                        continue;
                    }

                    let tile = tile - 1;

                    let tile_sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handle.clone(),
                        sprite_number: tile as usize,
                    };

                    let mut tile_transform = Transform::default();
                    let x_coord = x * tile_width as usize;
                    // Bottom Left is 0,0 so we flip it to Top Left with the
                    // ScreenDimensions.height since tiled coordinates start from top
                    let y_coord = (screen_height) - (y as f32 * tile_height as f32);
                    // Offset the positions by half the tile size so they're nice and snuggly on the screen
                    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                    let offset_x = tile_width as f32/2.0;
                    let offset_y = -tile_height as f32/2.0;

                    tile_transform.set_translation_xyz(
                        offset_x + x_coord as f32,
                        offset_y + y_coord as f32,
                        1.0
                    );

                    world
                        .create_entity()
                            .with(tile_transform)
                            .with(tile_sprite)
                        .build();
                }

            }
        }
    }
}

fn main() -> Result<(), amethyst::Error> {

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 0.0, 0.0, 1.0]),
                    )
                .with_plugin(RenderFlat2D::default())
            )?;

    let mut game = Application::new(app_root.join("assets"), GameplayState, game_data)?;

    game.run();
    
    Ok(())
}