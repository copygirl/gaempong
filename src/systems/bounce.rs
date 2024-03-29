use amethyst::{
    core::{Float, Transform},
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Ball, Paddle, Side};
use crate::pong::ARENA_HEIGHT;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );
   
    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent
        // multiple collisions from occurring.
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            
            // Bounce at the top or the bottom of the arena.
            if (ball_y.as_f32() <=                ball.radius && ball.velocity[1] < 0.0)
            || (ball_y.as_f32() >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            }
            
            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - Float::from(paddle.width  * 0.5);
                let paddle_y = paddle_transform.translation().y - Float::from(paddle.height * 0.5);
                
                // To determine whether the ball has collided with a paddle, we
                // create a larger rectangle around the current one, by subtracting
                // the ball radius from the lowest coordinates, and adding the ball
                // radius to the highest ones. The ball is then within the paddle
                // if its center is within the larger wrapper rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius.into(),
                    paddle_y - ball.radius.into(),
                    paddle_x + (paddle.width  + ball.radius).into(),
                    paddle_y + (paddle.height + ball.radius).into(),
                ) {
                    if (paddle.side == Side::Left  && ball.velocity[0] < 0.0)
                    || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: Float, y: Float, left: Float, bottom: Float, right: Float, top: Float) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
