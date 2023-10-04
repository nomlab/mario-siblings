// Written in Bevy 0.9.0dev

use bevy::{prelude::*, time::FixedTimestep, window};

const TIME_STEP: f32 = 1.0 / 60.0;

const UNIT: f32 = 10.0;

const VISIBLE_STAGE_WIDTH: isize = 32;
const WHOLE_STAGE_WIDTH: isize = 36;
const VISIBLE_STAGE_HEIGHT: isize = 25;
const WHOLE_STAGE_HEIGHT: isize = 25;

const SCREEN_WIDTH: f32 = UNIT * VISIBLE_STAGE_WIDTH as f32;
const SCREEN_HEIGHT: f32 = UNIT * VISIBLE_STAGE_HEIGHT as f32;

const BLOCK_SIZE: Vec3 = Vec3::new(UNIT, UNIT, 0.0);
const MARIO_SIZE: Vec3 = Vec3::new(UNIT * 2.0, UNIT * 2.5, 0.0);

const WARP_X_BORDER: f32 = SCREEN_WIDTH / 2.0 + MARIO_SIZE.x / 2.0;

const WALKING_SPEED: f32 = 2.0;
const JUMP_SPEED: f32 = 7.0;
const GRAVITY: f32 = 0.4;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
// const MARIO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const STAGE: [&str; 25] = [
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "XXXXXXXXXXXXXXX]____[XXXXXXXXXXXXXXX",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "__________[XXXXXXXXXXXXXX]__________",
    "XXXXX]________________________[XXXXX",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "XXXXXXXXXXXXX]____M___[XXXXXXXXXXXXX",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "____________________________________",
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
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
}

impl Block {
    fn normal_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: false,
            collision_left: false,
        }
    }

    fn left_edge_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: false,
            collision_left: true,
        }
    }
    fn right_edge_block() -> Self {
        Self {
            collision_top: true,
            collision_bottom: true,
            collision_right: true,
            collision_left: false,
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
            let x_coord = (j as isize - (WHOLE_STAGE_WIDTH / 2)) as f32 * UNIT + 0.5 * UNIT;
            let y_coord = (-(i as isize) + (WHOLE_STAGE_HEIGHT / 2)) as f32 * UNIT;
            match c {
                b'X' => spawn_block(&mut commands, Block::normal_block(), x_coord, y_coord),
                b']' => spawn_block(&mut commands, Block::right_edge_block(), x_coord, y_coord),
                b'[' => spawn_block(&mut commands, Block::left_edge_block(), x_coord, y_coord),
                b'M' => spawn_mario(&mut commands, &asset_server, x_coord, y_coord),
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

fn calc_velocity(
    mario_on_ground: bool,
    keyboard_input: Res<Input<KeyCode>>,
    mario_velocity: &mut Vec2,
) -> (f32, f32) {
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
        if keyboard_input.pressed(KeyCode::Up) {
            dy = JUMP_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {}
    }
    (dx, dy)
}

fn update_position(
    mut mario_query: Query<(&Velocity, &mut Transform, &mut Mario), With<Mario>>,
    block_query: Query<(&Transform, &Block), Without<Mario>>,
) {
    let (mario_velocity, mut mario_transform, mut mario) = mario_query.single_mut();
    let current_x_pos = mario_transform.translation.x;
    let current_y_pos = mario_transform.translation.y;

    let mario_pos = Vec3::new(
        current_x_pos + mario_velocity.x,
        current_y_pos + mario_velocity.y,
        0.0,
    );
    let (dx, dy, is_on_ground) =
        check_for_collisions(mario_pos, MARIO_SIZE, block_query, mario_velocity);

    mario.is_on_ground = is_on_ground;
    mario_transform.translation.x = mario_pos.x + dx;
    mario_transform.translation.y = mario_pos.y + dy;

    // When mario reaches edge of screen, warp to the opposite edge
    if mario_transform.translation.x.abs() > WARP_X_BORDER {
        mario_transform.translation.x *= -0.99
    }
}

fn check_for_collisions(
    mario_pos: Vec3,
    mario_size: Vec3,
    block_query: Query<(&Transform, &Block), Without<Mario>>,
    mario_velocity: &Velocity,
) -> (f32, f32, bool) {
    let mario_size = mario_size.truncate();
    let (mut dx, mut dy) = (0.0, 0.0);
    let mut is_on_ground = false;

    for (transform, block) in &block_query {
        if let Some(collision) = collide_standback(
            transform.translation,
            transform.scale.truncate(),
            mario_pos,
            mario_size,
            mario_velocity,
        ) {
            match collision {
                Collision(Surface::Top, factor) => {
                    if block.collision_top {
                        println!(
                            "Collision: {:?} mario_velocity: {},{}",
                            collision, mario_velocity.x, mario_velocity.y
                        );
                        dx = 0.0;
                        dy = -mario_velocity.y * (1.0 - factor);
                        is_on_ground = true;
                    }
                }
                Collision(Surface::Bottom, factor) => {
                    if block.collision_bottom {
                        println!(
                            "Collision: {:?} mario_velocity: {},{}",
                            collision, mario_velocity.x, mario_velocity.y
                        );
                        dx = 0.0;
                        dy = -mario_velocity.y * (1.0 - factor);
                    }
                }
                Collision(Surface::Left, factor) => {
                    if block.collision_left {
                        println!(
                            "Collision: {:?} mario_velocity: {},{}",
                            collision, mario_velocity.x, mario_velocity.y
                        );
                        dx = -mario_velocity.x * (1.0 - factor);
                        dy = 0.0;
                    }
                }
                Collision(Surface::Right, factor) => {
                    if block.collision_right {
                        dx = -mario_velocity.x * (1.0 - factor);
                        println!(
                            "Collision: {:?} mario_velocity: {},{}",
                            collision, mario_velocity.x, mario_velocity.y
                        );
                        dy = 0.0;
                    }
                }
                Collision(Surface::None, _) => {
                    // Do nothing
                }
            }
        }
    }
    println!("(dx: {}, dy: {}, is_on_ground: {})", dx, dy, is_on_ground);
    (dx, dy, is_on_ground)
}

#[derive(Debug)]
enum Surface {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Collision(Surface, f32);

fn collide_standback(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    b_velocity: &Velocity,
) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;
    let b_vx = b_velocity.x;
    let b_vy = b_velocity.y;

    // a A b B or b B a A
    if a_max.x <= b_min.x || b_max.x <= a_min.x {
        return None;
    }
    // a A b B or b B a A
    if a_max.y <= b_min.y || b_max.y <= a_min.y {
        return None;
    }

    let (x_direction, dx) = if b_vx > 0.0 {
        // bumped A's Left: b a-B A
        (Surface::Left, b_max.x - a_min.x)
    } else if b_vx < 0.0 {
        // bumped A's Right: a b-A B
        (Surface::Right, a_max.x - b_min.x)
    } else {
        (Surface::None, 0.0)
    };

    let (y_direction, dy) = if b_vy > 0.0 {
        // bumped A's Bottom:
        // A
        // B
        // |
        // a
        // b
        (Surface::Bottom, b_max.y - a_min.y)
    } else if b_vy < 0.0 {
        // bumped A's Top:
        // B
        // A
        // |
        // b
        // a
        (Surface::Top, a_max.y - b_min.y)
    } else {
        (Surface::None, 0.0)
    };

    let x_factor = (b_vx.abs() - dx.abs()) / b_vx.abs();
    let y_factor = (b_vy.abs() - dy.abs()) / b_vy.abs();

    let (direction, factor) = if x_factor > y_factor {
        (x_direction, x_factor)
    } else {
        (y_direction, y_factor)
    };

    Some(Collision(direction, factor))
}
