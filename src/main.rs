use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::state::app::AppExtStates;
use bevy::state::condition::in_state;
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::time::Duration;

const GRAVITY: f32 = -500.0;
const BIRD_JUMP: f32 = 300.0;
const PIPE_SPEED: f32 = 150.0;
const PIPE_GAP: f32 = 200.0;
const PIPE_SPAWN_INTERVAL: f32 = 2.0;
const GROUND_HEIGHT: f32 = -250.0;
const SPEED_INCREASE_RATE: f32 = 0.05; // Speed multiplier increase per pipe passed
const MAX_SPEED_MULTIPLIER: f32 = 2.5; // Maximum speed multiplier

#[derive(Component)]
struct Bird {
    velocity: f32,
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct Velocity {
    x: f32,
}

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Resource)]
struct GameDifficulty {
    speed_multiplier: f32,
    pipes_passed: u32,
}

impl Default for GameDifficulty {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            pipes_passed: 0,
        }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rlappy Bird".to_string(),
                        resolution: (800.0, 600.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(Score(0))
        .insert_resource(GameDifficulty::default())
        .insert_resource(PipeSpawnTimer(Timer::from_seconds(
            PIPE_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, menu_system.run_if(in_state(GameState::Menu)))
        .add_systems(
            Update,
            (
                bird_movement,
                bird_input,
                pipe_movement,
                spawn_pipes,
                check_collisions,
                update_score,
                execute_animations,
                pause_input,
                screenshot_input,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (pause_input, screenshot_input, unpause_system)
                .run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            Update,
            game_over_system.run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn UI text
    commands.spawn((TextBundle::from_section(
        "Press SPACE to Start",
        TextStyle {
            font_size: 40.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(50.0),
        left: Val::Px(250.0),
        ..default()
    }),));
}

fn menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<Entity, With<Text>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Clear menu text
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        // Load the bird sprite sheet
        let texture = asset_server.load("bird.png");

        // The sprite sheet has 4 sprites in a 2x2 grid
        // Each sprite is 512x512 (since the image is 1024x1024)
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(512), 2, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        // Create animation config - animate through all 4 frames at 8 FPS
        let animation_config = AnimationConfig::new(0, 3, 8);

        // Spawn bird with animated sprite
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_scale(Vec3::splat(0.06))
                    .with_translation(Vec3::new(-100.0, 0.0, 0.0)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            },
            Bird { velocity: 0.0 },
            animation_config,
        ));

        // Spawn score text
        commands.spawn((TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),));

        // Spawn ground
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(1000.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 0.0),
            ..default()
        });

        next_state.set(GameState::Playing);
    }
}

fn bird_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Bird>) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut bird in query.iter_mut() {
            bird.velocity = BIRD_JUMP;
        }
    }
}

fn bird_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Bird)>) {
    for (mut transform, mut bird) in query.iter_mut() {
        bird.velocity += GRAVITY * time.delta_seconds();
        transform.translation.y += bird.velocity * time.delta_seconds();
    }
}

// This system loops through all the sprites in the TextureAtlas
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and continue
                atlas.index = config.first_sprite_index;
                // Reset the timer to continue the loop
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

fn spawn_pipes(time: Res<Time>, mut timer: ResMut<PipeSpawnTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let gap_y = rng.gen_range(-150.0..150.0);

        // Spawn top pipe
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(60.0, 400.0)),
                    ..default()
                },
                transform: Transform::from_xyz(500.0, gap_y + PIPE_GAP / 2.0 + 200.0, 0.0),
                ..default()
            },
            Pipe,
            Velocity { x: -PIPE_SPEED },
        ));

        // Spawn bottom pipe
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(60.0, 400.0)),
                    ..default()
                },
                transform: Transform::from_xyz(500.0, gap_y - PIPE_GAP / 2.0 - 200.0, 0.0),
                ..default()
            },
            Pipe,
            Velocity { x: -PIPE_SPEED },
        ));
    }
}

fn pipe_movement(
    time: Res<Time>,
    difficulty: Res<GameDifficulty>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Pipe>>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds() * difficulty.speed_multiplier;

        // Despawn pipes that are off screen
        if transform.translation.x < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for bird_transform in bird_query.iter() {
        // Check ground collision
        if bird_transform.translation.y < GROUND_HEIGHT + 25.0 {
            next_state.set(GameState::GameOver);
            return;
        }

        // Check ceiling collision
        if bird_transform.translation.y > 300.0 {
            next_state.set(GameState::GameOver);
            return;
        }

        // Check pipe collision
        for pipe_transform in pipe_query.iter() {
            let bird_pos = bird_transform.translation;
            let pipe_pos = pipe_transform.translation;

            // Simple AABB collision
            if (bird_pos.x - pipe_pos.x).abs() < 45.0 && (bird_pos.y - pipe_pos.y).abs() < 215.0 {
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

fn update_score(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    mut score: ResMut<Score>,
    mut difficulty: ResMut<GameDifficulty>,
    mut text_query: Query<&mut Text>,
) {
    for bird_transform in bird_query.iter() {
        for pipe_transform in pipe_query.iter() {
            // Check if bird passed a pipe
            if bird_transform.translation.x > pipe_transform.translation.x
                && bird_transform.translation.x < pipe_transform.translation.x + 5.0
            {
                score.0 += 1;
                
                // Increase difficulty every 2 pipes (1 complete gap)
                if score.0 % 2 == 0 {
                    difficulty.pipes_passed += 1;
                    difficulty.speed_multiplier = (1.0 + SPEED_INCREASE_RATE * difficulty.pipes_passed as f32)
                        .min(MAX_SPEED_MULTIPLIER);
                }
                
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = format!("Score: {}", score.0 / 2);
                }
            }
        }
    }
}

fn game_over_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    entities_query: Query<Entity, Or<(With<Bird>, With<Pipe>, With<Text>, With<Sprite>)>>,
    text_count_query: Query<&Text>,
    mut score: ResMut<Score>,
    mut difficulty: ResMut<GameDifficulty>,
) {
    // Spawn game over text on first frame
    if text_count_query.iter().count() == 1 {
        commands.spawn((TextBundle::from_section(
            format!("Game Over! Score: {}\nPress R to Restart", score.0 / 2),
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(200.0),
            ..default()
        }),));
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        // Clean up all entities
        for entity in entities_query.iter() {
            commands.entity(entity).despawn();
        }

        // Reset score and difficulty
        score.0 = 0;
        *difficulty = GameDifficulty::default();

        // Go back to menu
        next_state.set(GameState::Menu);

        // Setup menu again
        commands.spawn((TextBundle::from_section(
            "Press SPACE to Start",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(250.0),
            ..default()
        }),));
    }
}

fn pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        // Spawn pause text
        commands.spawn((TextBundle::from_section(
            "PAUSED\nPress P to Resume",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(250.0),
            left: Val::Px(280.0),
            ..default()
        }),));
        
        next_state.set(GameState::Paused);
    }
}

fn unpause_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    text_query: Query<Entity, With<Text>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        // Remove pause text (keep only score text)
        let mut count = 0;
        for entity in text_query.iter() {
            if count > 0 {
                commands.entity(entity).despawn();
            }
            count += 1;
        }
        
        next_state.set(GameState::Playing);
    }
}

fn screenshot_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        let path = format!("./screenshot-{}.png", chrono::Local::now().format("%Y%m%d-%H%M%S"));
        
        if let Ok(entity) = main_window.get_single() {
            screenshot_manager
                .save_screenshot_to_disk(entity, path)
                .unwrap();
            println!("Screenshot saved!");
        }
    }
}

