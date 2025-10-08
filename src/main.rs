use bevy::prelude::*;
use bevy::state::app::AppExtStates;
use bevy::state::condition::in_state;
use rand::Rng;

const GRAVITY: f32 = -500.0;
const BIRD_JUMP: f32 = 300.0;
const PIPE_SPEED: f32 = 150.0;
const PIPE_GAP: f32 = 200.0;
const PIPE_SPAWN_INTERVAL: f32 = 2.0;
const GROUND_HEIGHT: f32 = -250.0;

#[derive(Component)]
struct Bird {
    velocity: f32,
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

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rlappy Bird".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Score(0))
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
            )
                .run_if(in_state(GameState::Playing)),
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
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Clear menu text
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        // Spawn bird
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-100.0, 0.0, 0.0),
                ..default()
            },
            Bird { velocity: 0.0 },
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
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Pipe>>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();

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
    mut text_query: Query<&mut Text>,
) {
    for bird_transform in bird_query.iter() {
        for pipe_transform in pipe_query.iter() {
            // Check if bird passed a pipe
            if bird_transform.translation.x > pipe_transform.translation.x
                && bird_transform.translation.x < pipe_transform.translation.x + 5.0
            {
                score.0 += 1;
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

        // Reset score
        score.0 = 0;

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
