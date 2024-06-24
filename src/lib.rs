use bevy::{
    prelude::{Component, Deref, DerefMut, Entity, Resource},
    render::color::Color,
    time::Timer,
};

// 常量，蛇头颜色
pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Resource)]
pub struct SnakeTimer(pub Timer);

// 蛇头
#[derive(Component)]
pub struct SnakeHead {
    pub direction: SnakeMoveDirection,
}

// 蛇运动方向
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SnakeMoveDirection {
    Init,
    Left,
    Up,
    Right,
    Down,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);
