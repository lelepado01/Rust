use std::fs::File;
use std::process::exit;

use ron::de::from_reader;
use serde::*;

use amethyst::{
    prelude::*,
    core::transform::Transform, 
    renderer::Camera
};

#[derive(Deserialize)]
pub struct CameraConfig {
    translation_x : f32, 
    translation_y : f32, 
    translation_z : f32, 
    camera_dim_x : f32, 
    camera_dim_y : f32
}

pub fn initialize_camera(world: &mut World){

    let file = File::open("config/camera_config.ron").expect("File not Found");
    let camera_config : CameraConfig = match from_reader(file) {
        Ok(x) => x, 
        Err(y) => {
            println!("Wrong Configs!");
            exit(1);
        }
            
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(
        camera_config.translation_x, 
        camera_config.translation_y, 
        camera_config.translation_z
    );

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_3d(camera_config.camera_dim_x, camera_config.camera_dim_y))
        .build();
}