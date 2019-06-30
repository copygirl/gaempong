use std::time::Duration;

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, System, WriteStorage},
};

use crate::components::Ball;
use crate::pong::ARENA_WIDTH;

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );
    
    fn run(&mut self, (entities, mut balls, mut transforms): Self::SystemData) {
        let num_balls = (&balls).join().count();
        for (entity, ball, transform) in (&*entities, &mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;
            
            let did_hit = if ball_x.as_f32() <= ball.radius {
                // Right player scored on the left side.
                println!("Player 2 Scores!");
                true
            } else if ball_x.as_f32() >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                println!("Player 1 Scores!");
                true
            } else {
                false
            };
            
            if did_hit {
                if num_balls > 1 {
                    entities.delete(entity).unwrap();
                } else {
                    ball.velocity[0] *= -1.0; // Reverse Direction
                    ball.age = Duration::default(); // Reset Age
                    transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                }
            }
        }
    }
}
