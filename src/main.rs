use bevy::prelude::*;

// Constants

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

const BRICK_LINES: f32 = 4.0;
const BRICK_COLUMNS: f32 = 16.0;

const BRICK_BEGIN_X: f32 = -300.0;
const BRICK_BEGIN_Y: f32 = 150.0;
const BRICK_SIZE: Vec2 = Vec2::new(32.0, 20.0);
const BRICK_SPACES: f32 = 8.0;


// States

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    Playing,
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

// Main

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
    .add_state(GameState::MainMenu)
    .insert_resource(GameData { score: 0, lifes: 3 })
    .add_startup_system(setup_system)
    .add_startup_system(spawn_bricks_system)
    .run();
}

fn setup_system(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default()); //Camera
}


fn spawn_bricks_system(
    mut commands: Commands,
) {
    for x in 0..BRICK_COLUMNS as u32 {
        for y in 0..BRICK_LINES as u32 {
            let pos_x = BRICK_BEGIN_X + (BRICK_SPACES+BRICK_SIZE.x) * x as f32;
            let pos_y = BRICK_BEGIN_Y + (BRICK_SPACES+BRICK_SIZE.y) * y as f32;

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(BRICK_SIZE.clone()),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.0)),
                ..Default::default()
            })
            .insert(Brick);
        }
    }
}