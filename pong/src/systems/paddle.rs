
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>, 
        ReadStorage<'s, Paddle>, 
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, paddles, input) : Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let paddle_y = transform.translation().y;
                    let scaled_movement = 1.2 * mv_amount as f32;
                    transform.set_translation_y(
                        (paddle_y + scaled_movement)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5));
                }
            }
        }
    }
}