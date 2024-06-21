use bevy::prelude::*;

// 常量，蛇头颜色
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

// 结构体，蛇头
#[derive(Component)]
struct SnakeHead;

// 主函数
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_snake))
        .run();
}

// 设置摄像机
fn setup_camera(mut commands: Commands) {
    //Pushes a Command to the queue for creating a new entity with the given Bundle's components, and returns its corresponding EntityCommands.

    commands.spawn(Camera2dBundle::default());
}

// 设置蛇头
fn spawn_snake(mut commands: Commands) {
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
    commands.spawn(sprite_bundle).insert(SnakeHead);
}
