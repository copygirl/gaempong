use std::time::Duration;

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use crate::components::Ball;

pub const BALL_FREEZE_TIME: Duration = Duration::from_secs(2);

pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );
   
    fn run(&mut self, (mut balls, mut transforms, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            ball.age += time.delta_time();
            if ball.age >= BALL_FREEZE_TIME
            {
                transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
                transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
            }
        }
    }
}
