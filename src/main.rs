// Written in Bevy 0.9.0dev

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    window,
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
// const MARIO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const STAGE: [&str; 25] = [
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "XXXXXXXXXXXXXX]____[XXXXXXXXXXXXXX",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________[XXXXXXXXXXXXXX]________|",
    "XXXX]________________________[XXXX",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "XXXXXXXXXXXX]________[XXXXXXXXXXXX",
    "|________________________________|",
    "|________________M_______________|",
    "|________________________________|",
    "|________________________________|",
    "|________________________________|",
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
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
        .add_system(window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Block {
    collision_top: bool,
    collision_bottom: bool,
    collision_right: bool,
    collision_left: bool,
    outside: bool,
}

impl Block {
    fn normal_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: false,
            collision_left: false,
            outside: false,
        }
    }

    fn left_edge_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: false,
            collision_left: true,
            outside: false,
        }
    }
    fn right_edge_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: true,
            collision_left: false,
            outside: false,
        }
    }
    fn outside_block() -> Self {
        Self {
            collision_top: false,
            collision_bottom: false,
            collision_right: false,
            collision_left: false,
            outside: true,
        }
    }
}

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

fn make_stage(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (i, stage_row) in STAGE.iter().enumerate() {
        for (j, c) in stage_row.as_bytes().iter().enumerate() {
            let x_coord =
                (j as isize - STAGE_X_OFFSET - (STAGE_WIDTH / 2)) as f32 * UNIT + 0.5 * UNIT;
            let y_coord = (-(i as isize) + (STAGE_HEIGHT / 2)) as f32 * UNIT;
            match c {
                b'X' => spawn_block(&mut commands, Block::normal_block(), x_coord, y_coord),
                b']' => spawn_block(&mut commands, Block::right_edge_block(), x_coord, y_coord),
                b'[' => spawn_block(&mut commands, Block::left_edge_block(), x_coord, y_coord),
                b'M' => spawn_mario(&mut commands, &asset_server, x_coord, y_coord),
                b'|' => spawn_block(&mut commands, Block::outside_block(), x_coord, y_coord),
                _ => {}
            };
        }
    }
}

fn spawn_block(commands: &mut Commands, block: Block, x_coord: f32, y_coord: f32) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x_coord, y_coord, 0.0),
                scale: BLOCK_SIZE,
                ..default()
            },
            ..default()
        },
        block,
    ));
}

fn spawn_mario(commands: &mut Commands, asset: &Res<AssetServer>, x_coord: f32, y_coord: f32) {
    let texture: Handle<Image> = asset.load("moo.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x_coord, y_coord, 0.0),
                scale: MARIO_SIZE,
                ..default()
            },
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
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

    for (transform, block) in &block_query {
        let mario_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            mario_transform.translation,
            mario_size,
        );
        if let Some(collision) = mario_collision {
            collision_events.send_default();

            match collision {
                Collision::Left => {
                    if block.collision_right {
                        mario_transform.translation.x = transform.translation.x
                            + transform.scale.truncate().x / 2.0
                            + mario_size.x / 2.0;
                    }
                }
                Collision::Right => {
                    if block.collision_left {
                        mario_transform.translation.x = transform.translation.x
                            - transform.scale.truncate().x / 2.0
                            - mario_size.x / 2.0;
                    }
                }
                Collision::Top => {
                    if block.collision_bottom {
                        mario_transform.translation.y = transform.translation.y
                            - transform.scale.truncate().y / 2.0
                            - mario_size.y / 2.0;
                    }
                }
                Collision::Bottom => {
                    if block.collision_top {
                        if mario_velocity.y <= 0.0 {
                            mario_velocity.x = 0.0;
                            mario_velocity.y = 0.0;
                            mario_transform.translation.y = transform.translation.y
                                + transform.scale.truncate().y / 2.0
                                + mario_size.y / 2.0;
                        }
                        mario.is_on_ground = true;
                    }
                }
                Collision::Inside => {
                    let mario_x_position = mario_transform.translation.x;
                    if block.outside {
                        if mario_x_position.is_sign_positive() {
                            mario_transform.translation.x = -(mario_x_position - MARIO_SIZE.x / 2.0)
                        } else {
                            mario_transform.translation.x = -(mario_x_position + MARIO_SIZE.x / 2.0)
                        }
                    }
                }
            }
        }
    }
}
