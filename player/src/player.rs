
use amethyst::{
    core::transform::Transform, 
    ecs::{Component, DenseVecStorage}, 
    assets::Handle
};

pub struct Player{
    pub position : Transform
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
