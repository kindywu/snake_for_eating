use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use rand::prelude::random;
use snake_for_eating::{
    Position, Size, SnakeHead, SnakeMoveDirection, SnakeSegment, SnakeSegments, SnakeTimer,
    ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR,
};

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
        .insert_resource(SnakeTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert_resource(SnakeSegments::default())
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
fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    // A Bundle of components for drawing a single sprite from an image.

    *segments = SnakeSegments(vec![
        // Adds a Bundle of components to the entity
        spawn_head(&mut commands, Position { x: 3, y: 3 }),
        spawn_segment(&mut commands, Position { x: 3, y: 2 }),
    ]);
}

fn spawn_head(commands: &mut Commands, position: Position) -> Entity {
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

    commands
        .spawn(sprite_bundle)
        .insert(SnakeHead {
            direction: SnakeMoveDirection::Init,
        })
        .insert(position)
        .insert(Size::square(0.8))
        .id()
}

fn spawn_segment(commands: &mut Commands, position: Position) -> Entity {
    // 创建精灵
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        },
        ..default()
    };
    commands
        .spawn(sprite_bundle)
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    for mut head in heads.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            head.direction = SnakeMoveDirection::Left;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            head.direction = SnakeMoveDirection::Right;
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            head.direction = SnakeMoveDirection::Up;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            head.direction = SnakeMoveDirection::Down;
        }
    }
}

// 蛇头移动
fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    segments: ResMut<SnakeSegments>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        // info!("{:?}", &head.direction);
        match &head.direction {
            SnakeMoveDirection::Init => (),
            SnakeMoveDirection::Left => head_pos.x -= 1,
            SnakeMoveDirection::Up => head_pos.y += 1,
            SnakeMoveDirection::Right => head_pos.x += 1,
            SnakeMoveDirection::Down => head_pos.y -= 1,
        };

        if head.direction != SnakeMoveDirection::Init {
            segment_positions
                .iter()
                .zip(segments.iter().skip(1))
                .for_each(|(pos, segment)| {
                    *positions.get_mut(*segment).unwrap() = *pos;
                })
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
