// Written in Bevy 0.9.0dev

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const UNIT: f32 = 10.0;

const STAGE_WIDTH: isize = 32;
const STAGE_HEIGHT: isize = 25;
const STAGE_X_OFFSET: isize = 1;

const SCREEN_WIDTH: f32 = UNIT * STAGE_WIDTH as f32;
const SCREEN_HEIGHT: f32 = UNIT * STAGE_HEIGHT as f32;

const BLOCK_SIZE: Vec3 = Vec3::new(10.0, 10.0, 0.0);
const MARIO_SIZE: Vec3 = Vec3::new(UNIT * 2.0, UNIT * 2.5, 0.0);

const WALKING_SPEED: f32 = 2.0;
const JUMP_SPEED: f32 = 7.0;
const GRAVITY: f32 = 0.4;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const MARIO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const STAGE: [&str; 25] = [
    "|XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|XXXXXXXXXXXXXX____XXXXXXXXXXXXXX|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________XXXXXXXXXXXXXXXX________|",
    "|XXXX________________________XXXX|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|XXXXXXXXXXXX________XXXXXXXXXXXX|",
    "|________________________________|",
    "|________________M_______________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX|",
];

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
        .add_startup_system(make_stage)
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
struct Mario {
    is_on_ground: bool,
}

#[derive(Default)]
struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn make_stage(mut commands: Commands) {
    for (i, stage_row) in STAGE.iter().enumerate() {
        for (j, c) in stage_row.as_bytes().iter().enumerate() {
            let x_coord =
                (j as isize - STAGE_X_OFFSET - (STAGE_WIDTH / 2)) as f32 * UNIT + 0.5 * UNIT;
            let y_coord = (-(i as isize) + (STAGE_HEIGHT / 2)) as f32 * UNIT;
            match c {
                b'X' => spawn_block(&mut commands, x_coord, y_coord),
                b'M' => spawn_mario(&mut commands, x_coord, y_coord),
                b'|' => spawn_block(&mut commands, x_coord, y_coord),
                _ => {}
            };
        }
    }
}

fn spawn_block(commands: &mut Commands, x_coord: f32, y_coord: f32) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x_coord, y_coord, 0.0),
                scale: BLOCK_SIZE,
                ..default()
            },
            ..default()
        },
        Block,
    ));
}

fn spawn_mario(commands: &mut Commands, x_coord: f32, y_coord: f32) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x_coord, y_coord, 0.0),
                scale: MARIO_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: MARIO_COLOR,
                ..default()
            },
            ..default()
        },
        Mario {
            is_on_ground: false,
        },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

fn apply_velocity(
    keyboard_input: Res<Input<KeyCode>>,
    mut mario_query: Query<(&mut Velocity, &Mario), With<Mario>>,
) {
    let (mut mario_velocity, mario) = mario_query.single_mut();

    if mario.is_on_ground && mario_velocity.y == 0.0 {
        if keyboard_input.pressed(KeyCode::Left) {
            mario_velocity.x = -WALKING_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            mario_velocity.x = WALKING_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            mario_velocity.y = JUMP_SPEED;
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
    mario_velocity.y -= GRAVITY;
}

fn check_for_collisions(
    mut mario_query: Query<(&mut Velocity, &mut Transform, &mut Mario), Without<Block>>,
    block_query: Query<(&Transform, &Block), Without<Mario>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut mario_velocity, mut mario_transform, mut mario) = mario_query.single_mut();
    let mario_size = mario_transform.scale.truncate();
    mario.is_on_ground = false;

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
                    mario_transform.translation.x = transform.translation.x
                        - transform.scale.truncate().x / 2.0
                        - mario_size.x / 2.0;
                }
                Collision::Right => {
                    mario_transform.translation.x = transform.translation.x
                        + transform.scale.truncate().x / 2.0
                        + mario_size.x / 2.0;
                }
                Collision::Top => {
                    if mario_velocity.y <= 0.0 {
                        mario_velocity.x = 0.0;
                        mario_velocity.y = 0.0;
                        mario_transform.translation.y = transform.translation.y
                            + transform.scale.truncate().y / 2.0
                            + mario_size.y / 2.0;
                    }
                    mario.is_on_ground = true;
                }
                Collision::Bottom => {
                    mario_transform.translation.y = transform.translation.y
                        - transform.scale.truncate().y / 2.0
                        - mario_size.y / 2.0;
                }
                Collision::Inside => {}
            }
        }
    }
}
