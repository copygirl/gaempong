use std::time::Duration;
use amethyst::ecs::Component;
use amethyst::ecs::storage::DenseVecStorage;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
    pub age: Duration,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
