
use crate::player::Player;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, WriteStorage, System, SystemData}
};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (players, mut transforms) : Self::SystemData){
        for (player, transform) in (&players, &mut transforms).join() {
            transform.move_left(1.0);
            if transform.translation().x < 100.0 {
                transform.set_translation_x(400.0);
            }
        }
    }
}