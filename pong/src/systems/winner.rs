use amethyst::{
    core::transform::Transform,
    ecs::prelude::{System, Join, WriteStorage, Write, ReadExpect},
    ui::UiText
};

use crate::pong::{Ball, ARENA_WIDTH, ScoreBoard, ScoreText};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>, 
        Write<'s, ScoreBoard>, 
        ReadExpect<'s, ScoreText>
    );

    fn run(&mut self, (mut balls, mut transform, mut ui_text, mut scores, score_text) : Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transform).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right +1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }

                println!("Player 2 scores!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left +1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                
                println!("Player 1 scores");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0);
            }
        }
    }


}