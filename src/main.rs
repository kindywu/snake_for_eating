use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use rand::prelude::random;

// 常量，蛇头颜色
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
// 结构体，蛇头
#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

// 主函数
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: WindowResolution::new(500.0, 500.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(KeyboardInputTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(
            Update,
            (
                snake_movement_input.before(snake_movement),
                snake_movement,
                size_scaling,
                position_translation,
            ),
        )
        .add_systems(Update, food_spawner)
        .run();
}

// 设置摄像机
fn setup_camera(mut commands: Commands) {
    //Pushes a Command to the queue for creating a new entity with the given Bundle's components, and returns its corresponding EntityCommands.

    commands.spawn(Camera2dBundle::default());
}

// 设置蛇头
fn spawn_snake(mut commands: Commands) {
    // A Bundle of components for drawing a single sprite from an image.

    // 创建精灵
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    };
    // Adds a Bundle of components to the entity
    commands
        .spawn(sprite_bundle)
        .insert(SnakeHead {
            direction: Direction::Init,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

#[derive(Resource)]
struct KeyboardInputTimer(Timer);

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Init,
    Left,
    Up,
    Right,
    Down,
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    for mut head in heads.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            head.direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            head.direction = Direction::Right;
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            head.direction = Direction::Up;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            head.direction = Direction::Down;
        }
    }
}

// 蛇头移动
fn snake_movement(
    mut heads: Query<(&mut Position, &SnakeHead)>,
    time: Res<Time>,
    mut timer: ResMut<KeyboardInputTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some((mut pos, head)) = heads.iter_mut().next() {
        info!("{:?}", &head.direction);
        match &head.direction {
            Direction::Init => (),
            Direction::Left => pos.x -= 1,
            Direction::Up => pos.y += 1,
            Direction::Right => pos.x += 1,
            Direction::Down => pos.y -= 1,
        };
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = primary_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / (ARENA_WIDTH as f32) * window.width(),
            sprite_size.height / (ARENA_HEIGHT as f32) * window.height(),
            1.0,
        )
    }
}

fn position_translation(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (title_size / 2.)
    }

    let window = primary_query.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        )
    }
}

#[derive(Component)]
struct Food;

#[derive(Resource)]
struct FoodSpawnTimer(Timer);

fn food_spawner(mut commands: Commands, time: Res<Time>, mut timer: ResMut<FoodSpawnTimer>) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}
