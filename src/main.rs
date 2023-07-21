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
    "XXXXXXXXXXXX]____M___[XXXXXXXXXXXX",
    "|________________________________|",
    "|________________________________|",
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
                .with_system(update_velocity.before(update_position))
                .with_system(update_position),
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

// マリオの速度を計算して代入する
fn update_velocity(
    mut mario_query: Query<(&mut Velocity, &Mario), With<Mario>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut mario_velocity, mario) = mario_query.single_mut();

    let (dx, dy) = calc_velocity(mario.is_on_ground, keyboard_input, &mut mario_velocity);

    mario_velocity.x += dx;
    mario_velocity.y += dy;
}

fn calc_velocity(mario_on_ground: bool, keyboard_input: Res<Input<KeyCode>>, mario_velocity: &mut Vec2) -> (f32, f32) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    dy -= GRAVITY;

    if mario_on_ground {
        println!("mario on ground");
        mario_velocity.x = 0.0;
        mario_velocity.y = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            dx = -WALKING_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            dx = WALKING_SPEED;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            dy = JUMP_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {}
    }
    (dx, dy)
}

fn update_position(
    mut mario_query: Query<(&Velocity, &mut Transform, &mut Mario), With<Mario>>,
    block_query: Query<(&Transform, &Block), Without<Mario>>) {
    let (mario_velocity, mut mario_transform, mut mario) = mario_query.single_mut();
    let current_x_pos = mario_transform.translation.x;
    let current_y_pos = mario_transform.translation.y;

    let mario_pos = Vec3::new(
        current_x_pos + mario_velocity.x,
        current_y_pos + mario_velocity.y,
        0.0,
    );
    let (dx, dy, is_on_ground) = check_for_collisions(mario_pos, MARIO_SIZE, block_query, mario_velocity);

    mario.is_on_ground = is_on_ground;
    mario_transform.translation.x = mario_pos.x + dx;
    mario_transform.translation.y = mario_pos.y + dy;
}

fn check_for_collisions(
    mario_pos: Vec3,
    mario_size: Vec3,
    block_query: Query<(&Transform, &Block), Without<Mario>>,
    mario_velocity: &Velocity
) -> (f32, f32, bool) {
    let mario_size = mario_size.truncate();
    let (mut dx, mut dy) = (0.0, 0.0);
    let mut is_on_ground = false;

    for (transform, block) in &block_query {
        let mario_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            mario_pos,
            mario_size,
        );
        if let Some(collision) = mario_collision {
            match collision {
                Collision::Left => {
                    if block.collision_right{
                        println!("collision_left");
                        (dx, _) = standback(transform.translation, transform.scale.truncate(), mario_pos, mario_size, mario_velocity)
                    }
                }
                Collision::Right => {
                    if block.collision_left{
                        println!("collision_right");
                        (dx, _) = standback(transform.translation, transform.scale.truncate(), mario_pos, mario_size, mario_velocity)
                    }
                }
                Collision::Top => {
                    if block.collision_bottom{
                        println!("collision_top");
                        (_, dy) = standback(transform.translation, transform.scale.truncate(), mario_pos, mario_size, mario_velocity)
                    }
                }
                Collision::Bottom => {
                    if block.collision_top{
                        println!("collision_bottom");
                        if mario_velocity.y < 0.0 {
                            is_on_ground = true;
                        }
                        (_, dy) = standback(transform.translation, transform.scale.truncate(), mario_pos, mario_size, mario_velocity)
                    }
                }
                Collision::Inside => {
                    let mario_x_position = mario_pos.x;
                    if block.outside {
                        if mario_x_position.is_sign_positive() {
                            (dx, dy) = (-(mario_x_position - MARIO_SIZE.x / 2.0) - mario_x_position, 0.0) 
                        } else {
                            (dx, dy) = (-(mario_x_position + MARIO_SIZE.x / 2.0) - mario_x_position, 0.0)
                        }
                    }
                }
            }
        }
    }
    println!("collision:({},{},{})", dx, dy, is_on_ground);
    (dx, dy, is_on_ground)
}

/// Axis-aligned bounding box collision with "side" detection
/// * `a_pos` and `b_pos` are the center positions of the rectangles, typically obtained by
/// extracting the `translation` field from a `Transform` component
/// * `a_size` and `b_size` are the dimensions (width and height) of the rectangles.
///
/// Returns Vec2 that means how `B` should stand-back if `B` has collided with `A`.
///
fn standback(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    b_velocity: &Velocity,
) -> (f32, f32) {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // a A b B or b B a A
    if a_max.x <= b_min.x || b_max.x <= a_min.x {
        return (0.0, 0.0);
    }
    // a A b B or b B a A
    if a_max.y <= b_min.y || b_max.y <= a_min.y {
        return (0.0, 0.0);
    }
    let x_setback = if b_velocity.x < 0.0 {
        (a_max.x - b_min.x).min(-b_velocity.x)
    } else {
        (a_min.x - b_max.x).min(-b_velocity.x)
    };
    let y_setback = if b_velocity.y < 0.0 {
        // a b A B
        // a b A B
        // これはプラスの値で，|b_velocity.x| 以下になる筈
        if (a_max.y - b_min.y) - (-b_velocity.y) > 0.001 {
            // もし，そうでないなら，x 方向の補正はいらない状況だと考える
            0.0
        } else {
            // (a_max.y - b_min.y).min(-b_velocity.y) 
            a_max.y - b_min.y 
        }
    } else {
        // b a B A
        // b a B A
        // これは，マイナスの値で，絶対値は b_velocity.x 以下になる筈
        if -(a_min.y - b_max.y) - b_velocity.y > 0.001 {
            // もし，そうでないなら，x 方向の補正はいらない状況だと考える
            0.0
        } else {
            a_min.y - b_max.y
        }
    };
    
    println!("velocity({}, {}), x_setback:{}, y_setback:{}", b_velocity.x, b_velocity.y, x_setback, y_setback);
    return (x_setback, y_setback);
}