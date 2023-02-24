// Written in Bevy 0.9.0dev

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const UNIT_WIDTH: u32 = 10;
const UNIT_HEIGHT: u32 = 10;

const X_LENGTH: u32 = 32;
const Y_LENGTH: u32 = 25;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

const BLOCK_SIZE: Vec3 = Vec3::new(10.0, 10.0, 0.0);
const MARIO_SIZE: Vec3 = Vec3::new(10.0, 20.0, 0.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const MARIO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_startup_system(make_field)
        .add_startup_system(make_mario)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(apply_velocity.before(move_mario))
                .with_system(move_mario.before(check_for_collisions)),
        )
        .run();
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Mario;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn make_mario(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -105.0, 0.0),
                scale: MARIO_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: MARIO_COLOR,
                ..default()
            },
            ..default()
        },
        Mario,
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

fn make_field(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut x_coord = -155.0;

    while x_coord < -155.0 + SCREEN_WIDTH as f32 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, -120.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    // 1段目
    x_coord = -155.0;
    while x_coord < -155.0 + UNIT_WIDTH as f32 * 12.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, -60.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    x_coord = 45.0;
    while x_coord < -155.0 + SCREEN_WIDTH as f32 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, -60.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    // 2段目
    x_coord = -155.0;
    while x_coord < -155.0 + UNIT_WIDTH as f32 * 4.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, -10.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    x_coord = -75.0;
    while x_coord < -155.0 + UNIT_WIDTH as f32 * 24.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, 0.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    x_coord = 125.0;
    while x_coord < -155.0 + SCREEN_WIDTH as f32 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, -10.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    // 3段目
    x_coord = -155.0;
    while x_coord < -155.0 + UNIT_WIDTH as f32 * 14.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, 60.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    x_coord = 25.0;
    while x_coord < -155.0 + SCREEN_WIDTH as f32 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x_coord, 60.0, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        x_coord = x_coord + 10.0;
    }

    // そり立つ壁
    let mut y_coord = -120.0;
    while y_coord < -120.0 + SCREEN_HEIGHT as f32 + 100.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-165.0, y_coord, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        y_coord = y_coord + 10.0;
    }

    y_coord = -120.0;
    while y_coord < -120.0 + SCREEN_HEIGHT as f32 + 100.0 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(165.0, y_coord, 0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                ..default()
            },
            Block,
        ));
        y_coord = y_coord + 10.0;
    }
}

fn is_on_ground(translation: Vec3) -> bool {
    println!("{}", translation.y);
    if translation.y == -105.0
        || (translation.y == -45.0 && (translation.x < -35.0 || 35.0 < translation.x))
        || (translation.y == 5.0 && (translation.x < -115.0 || 115.0 < translation.x))
        || (translation.y == 15.0 && (-85.0 < translation.x && translation.x < 85.0))
        || (translation.y == 75.0 && (translation.x < -15.0 || 15.0 < translation.x))
    {
        println!("on the ground!");
        return true;
    } else {
        println!("not on the ground!");
        return false;
    }
}

fn apply_velocity(
    keyboard_input: Res<Input<KeyCode>>,
    mut mario_query: Query<(&mut Velocity, &Transform), With<Mario>>,
) {
    let (mut mario_velocity, mario_transform) = mario_query.single_mut();

    if is_on_ground(mario_transform.translation) {
        if keyboard_input.pressed(KeyCode::Left) {
            mario_velocity.x = -2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            mario_velocity.x = 2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            mario_velocity.y = 3.5;
        }
        if keyboard_input.pressed(KeyCode::Down) {}
    }
}

fn move_mario(mut mario_query: Query<(&mut Velocity, &mut Transform), With<Mario>>) {
    let (mut mario_velocity, mut mario_transform) = mario_query.single_mut();
    let current_x_pos = mario_transform.translation.x;
    let current_y_pos = mario_transform.translation.y;

    mario_transform.translation.x = current_x_pos + mario_velocity.x;
    mario_transform.translation.y = current_y_pos + mario_velocity.y;
    // Apply gravity!
    mario_velocity.y -= 0.1;
}

fn check_for_collisions(
    mut mario_query: Query<(&mut Velocity, &mut Transform, &Mario), Without<Block>>,
    block_query: Query<(&Transform, &Block), Without<Mario>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut mario_velocity, mut mario_transform, _) = mario_query.single_mut();
    let mario_size = mario_transform.scale.truncate();

    for (transform, _) in &block_query {
        let collision = collide(
            mario_transform.translation,
            mario_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            collision_events.send_default();

            match collision {
                Collision::Left => {
                    mario_transform.translation.x = transform.translation.x - 10.0;
                }
                Collision::Right => {
                    mario_transform.translation.x = transform.translation.x + 10.0;
                }
                Collision::Top => {
                    mario_velocity.x = 0.0;
                    mario_velocity.y = 0.0;
                    // 地面に埋まるのを防ぐ
                    // TODO: 衝突せずに突き抜けた場合は想定していない
                    mario_transform.translation.y = transform.translation.y + 15.0;
                }
                Collision::Bottom => {
                    mario_transform.translation.y = transform.translation.y - 15.0;
                }
                Collision::Inside => {}
            }
        }
    }
}
