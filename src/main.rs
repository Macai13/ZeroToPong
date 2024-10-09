use bevy::prelude::*;
use iyes_perf_ui::entries::PerfUiBundle;
use iyes_perf_ui::prelude::*;
use rand::Rng;

const BALL_DIAMETER: f32 = 25.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 150.;

fn main() 
{
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)

        .add_systems(Startup, (spawn_camera, spawn_players, spawn_ball, spawn_background, spawn_fps))
        .add_systems(Update, (move_paddle, move_ball, ball_collide, ball_out_of_bounds))
        
        .run();
}

#[derive(Component)]
struct Paddle 
{
    move_up: KeyCode,
    move_down: KeyCode,
}

#[derive(Component)]
struct Ball(Vec2);

fn spawn_fps(mut commands: Commands)
{
    commands.spawn(PerfUiBundle::default());
}

fn spawn_camera(mut commands: Commands) 
{
    commands.spawn(Camera2dBundle::default());
}

fn spawn_background(mut commands: Commands)
{
    // Background
    commands.spawn(SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        sprite: Sprite 
        {
            color: Color::hsl(183., 0.78, 0.43),
            custom_size: Some(Vec2::new(1920., 1080.)),
            ..default()
        },
        ..default()
    });

    // Upper Wall
    commands.spawn(SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(0., 255., -1.)),
        sprite: Sprite 
        {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(650., 10.)),
            ..default()
        },
        ..default()
    });

    // Lower Wall
    commands.spawn(SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(0., -255., -1.)),
        sprite: Sprite 
        {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(650., 10.)),
            ..default()
        },
        ..default()
    });
}

fn spawn_players(mut commands: Commands)
{
    // Left Paddle
    commands.spawn((SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
        sprite: Sprite 
        {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        ..default()
    }, Paddle 
    { 
        move_up: KeyCode::KeyW, 
        move_down: KeyCode::KeyS,
    }));

    // Right Paddle
    commands.spawn((SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(300., 0., 0.)),
        sprite: Sprite 
        {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        ..default()
    }, Paddle 
    { 
        move_up: KeyCode::ArrowUp, 
        move_down: KeyCode::ArrowDown,
    }));
}

fn move_paddle(
    mut paddle: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
)
{
    for (mut pos, settings) in &mut paddle 
    {
        if input.pressed(settings.move_up)
        {
            pos.translation.y += 300. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-175., 175.);
        }

        if input.pressed(settings.move_down)
        {
            pos.translation.y -= 300. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-175., 175.);
        }
    }
}

fn spawn_ball(mut commands: Commands)
{
    // Ball
    commands.spawn((SpriteBundle 
    {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        sprite: Sprite 
        {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
            ..default()
        },
        ..default()
    }, Ball(Vec2::new(-100., 0.))));
}

fn move_ball(
    mut balls: Query<(&mut Transform, &Ball)>,
    time: Res<Time>
)
{
    for (mut pos, ball) in &mut balls
    { 
        pos.translation += ball.0.extend(0.) * time.delta_seconds();
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
)
{
    for (ball, mut velocity) in &mut balls
    {
        if ball.translation.y.abs() + BALL_DIAMETER / 2. > 250.
        {
            velocity.0.y *= -1.;
        }

        for paddle in &paddles 
        {
            if
            ball.translation.x - BALL_DIAMETER / 2. < paddle.translation.x + PADDLE_WIDTH / 2. &&
            ball.translation.y - BALL_DIAMETER / 2. < paddle.translation.y + PADDLE_HEIGHT / 2. &&
            ball.translation.x + BALL_DIAMETER / 2. > paddle.translation.x - PADDLE_WIDTH / 2. &&
            ball.translation.y + BALL_DIAMETER / 2. > paddle.translation.y - PADDLE_HEIGHT / 2.
            {
                println!("ball x: {}", velocity.0.x);
                println!("ball y: {}", velocity.0.y);

                if velocity.0.x.abs() < 2000.
                {
                    velocity.0.x *= -1.1;
                }
                else  
                {
                    velocity.0.x *= -1.;
                }
                velocity.0.y = rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<i8>() as f32;
                println!("Ball collided with paddle");
            }

            if ball.translation.y.abs() > 175.
            {
            }
        }
    }
}

fn ball_out_of_bounds(
    mut balls: Query<(&mut Transform, &mut Ball)>,
)
{
    for (mut pos, mut velocity) in &mut balls
    {
        if pos.translation.x < -325.
        {
            pos.translation = Vec3::new(0., 0., 0.);
            velocity.0.x = 100.; 
        }
        else if pos.translation.x > 325.
        {
            pos.translation = Vec3::new(0., 0., 0.);
            velocity.0.x = -100.; 
        }
    }
}