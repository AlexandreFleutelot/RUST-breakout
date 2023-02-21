use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

// Constants

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

const BRICK_BEGIN_X: f32 = -300.0;
const BRICK_BEGIN_Y: f32 = 150.0;
const BRICK_LINES: f32 = 4.0;
const BRICK_COLUMNS: f32 = 9.0;
const BRICK_SPACES: f32 = 8.0;

const BRICK_SPRITE: &str = "sprites/red_brick.png";
const BRICK_SIZE: Vec2 = Vec2::new(225.0, 76.0);
const BRICK_SCALE: Vec3 = Vec3::new(0.3,0.3,1.);

const PADDLE_SPRITES: &str = "sprites/paddle.png";
const PADDLE_SIZE: Vec2 = Vec2::new(202.0, 54.0);
const PADDLE_SCALE: Vec3 = Vec3::new(0.5,0.4,1.);
const PADDLE_Y_POS: f32 = -280.0;

const BALL_SPRITES: &str = "sprites/ball.png";
const BALL_SIZE: Vec2 = Vec2::new(97.0, 97.0);
const BALL_SCALE: Vec3 = Vec3::new(0.15,0.15,1.);
// States

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Start,
    Playing,
    Pause,
    GameOverScreen,
    WinScreen
}

// Resources

#[derive(Resource)]
struct GameData {
    score: u32,
    lifes: u32,
}

// Components

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    direction: Vec3,
    speed: f32,
}

fn main() {

    let window = WindowPlugin {
        window: WindowDescriptor {
            title: "Breakout".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()},
        ..Default::default() };

    App::new()
    .add_plugins(DefaultPlugins.set(window))
    .add_state(GameState::Start)
    .insert_resource(GameData { score: 0, lifes: 3 })
    .add_startup_system(setup_system)
    .add_system(game_screens_system)
    .add_system_set(
        SystemSet::on_enter(GameState::Start)
            .with_system(initialise_game_system))
    .add_system(ball_movement)
    .add_system(paddle_movement)
    .add_system(ball_collision)
    .run();
}

fn setup_system(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default()); //Camera
}

fn game_screens_system(
    kb: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    match game_state.current() {
        GameState::Start => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Playing).unwrap();
            }
        },
        GameState::Playing => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Pause).unwrap();
            }
        },
        GameState::Pause => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Playing).unwrap();
            }
        },
        GameState::WinScreen | GameState::GameOverScreen => {
            if kb.just_pressed(KeyCode::Space) {
                game_state.set(GameState::Start).unwrap();
            }
        }
    }
}

fn initialise_game_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    asset_server: Res<AssetServer>
) {

    //spawn bricks
    for x in 0..BRICK_COLUMNS as u32 {
        for y in 0..BRICK_LINES as u32 {
            let pos_x = BRICK_BEGIN_X + (BRICK_SPACES+BRICK_SIZE.x*BRICK_SCALE.x) * x as f32;
            let pos_y = BRICK_BEGIN_Y + (BRICK_SPACES+BRICK_SIZE.y*BRICK_SCALE.y) * y as f32;

            commands.spawn(SpriteBundle { 
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 0.0), 
                    rotation: Quat::IDENTITY, 
                    scale: BRICK_SCALE },
                texture: asset_server.load(BRICK_SPRITE),
                ..Default::default() 
            })
            .insert(Brick);
        }
    }

    //spawn paddle
    commands.spawn(SpriteBundle {
        transform: Transform { 
            translation: Vec3::new(0.0, PADDLE_Y_POS, 0.0), 
            rotation: Quat::IDENTITY, 
            scale: PADDLE_SCALE },
        texture: asset_server.load(PADDLE_SPRITES),
        ..Default::default()
    })
    .insert(Paddle);

    //spawn ball
    commands.spawn(SpriteBundle {
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 0.0), 
            rotation: Quat::IDENTITY, 
            scale: BALL_SCALE },
        texture: asset_server.load(BALL_SPRITES),
        ..Default::default()
    })
    .insert(Ball {direction: Vec3::new(1.0,1.0, 0.0), speed: 500.0 });

    //init life and score
    game_data.lifes = 3;
    game_data.score = 0;
}

fn ball_movement(
    mut ball_query: Query<(&mut Transform, &Ball), Without<Paddle>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    game_state: Res<State<GameState>>,
    time: Res<Time>
) {
    for (mut ball_tf, ball) in ball_query.iter_mut(){
        match game_state.current() {
            GameState::Start => {
                if let Ok(paddle_tf) = paddle_query.get_single() {
                    ball_tf.translation = paddle_tf.translation + Vec3::new(0.0, 20.0, 0.0);
                }
            },
            GameState::Playing => {
                let delta = time.delta().as_secs_f32();
                ball_tf.translation += ball.direction.normalize() * ball.speed * delta;
            }
            GameState::Pause => (),
            GameState::GameOverScreen => (),
            GameState::WinScreen => (),
        }
    }
}

fn paddle_movement(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    kb: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    time: Res<Time>
) {
    match game_state.current() {
        GameState::Start | GameState::Playing => {
            if let Ok(mut paddle_tf) = paddle_query.get_single_mut() {
                let delta = time.delta().as_secs_f32();
                if kb.pressed(KeyCode::Left) {
                    paddle_tf.translation += Vec3::new(-500.0,0.0,0.0) * delta;
                }
                if kb.pressed(KeyCode::Right) {
                    paddle_tf.translation += Vec3::new(500.0,0.0,0.0) * delta;
                }
                let limit = (WINDOW_WIDTH - PADDLE_SIZE.x * PADDLE_SCALE.x)/2.;
                paddle_tf.translation.x = paddle_tf.translation.x.clamp(-limit, limit);
            }
        },
        GameState::Pause => (),
        GameState::GameOverScreen => (),
        GameState::WinScreen => (),
    }

}

fn ball_collision(
    mut commands: Commands,
    paddle_query: Query<(&Transform, With<Paddle>), Without<Ball>>,
    brick_query: Query<(&Transform, Entity, With<Brick>), Without<Ball>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>
) {
    for (mut ball_tf, mut ball) in ball_query.iter_mut(){

        //right, left
        let border = WINDOW_WIDTH/2. - BALL_SIZE.x*BALL_SCALE.x/2.;
        if ball_tf.translation.x < -border || ball_tf.translation.x > border {
            ball.direction *= Vec3::new(-1.0,1.0,1.0);
        }

        //ceiling
        let ceiling = WINDOW_HEIGHT/2. - BALL_SIZE.x*BALL_SCALE.x/2.;
        if ball_tf.translation.y > ceiling {
            ball.direction *= Vec3::new(1.0,-1.0,1.0);
        }

        //paddle
        if let Ok((paddle_tf, _)) = paddle_query.get_single() {
            let collision = collide(
                ball_tf.translation,
                BALL_SIZE * Vec2::new(BALL_SCALE.x, BALL_SCALE.y),
                paddle_tf.translation,
                PADDLE_SIZE * Vec2::new(PADDLE_SCALE.x, PADDLE_SCALE.y)
            );
            if let Some(_) = collision {
                let delta = 2.*(ball_tf.translation.x - paddle_tf.translation.x) / (PADDLE_SIZE.x * PADDLE_SCALE.x);
                ball.direction.x += 0.5*delta;
                ball.direction.y *= -1.0;
                ball.direction = ball.direction.normalize();
            }
        }

        //bricks
        for (brick_tf, brick, _) in brick_query.iter() {
            let collision = collide(
                ball_tf.translation,
                BALL_SIZE * Vec2::new(BALL_SCALE.x, BALL_SCALE.y),
                brick_tf.translation,
                BRICK_SIZE * Vec2::new(BRICK_SCALE.x, BRICK_SCALE.y)
            );
            if let Some(_) = collision {
                commands.entity(brick).despawn();
                match collision.unwrap() {
                    Collision::Left | Collision::Right => ball.direction *= Vec3::new(-1.0,1.0,1.0),
                    Collision::Top | Collision::Bottom => ball.direction *= Vec3::new(1.0,-1.0,1.0),
                    Collision::Inside => (),
                }
            }
        }
    }
}