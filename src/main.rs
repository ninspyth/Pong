use bevy::prelude::*;
use::bevy::sprite::MaterialMesh2dBundle;
use::bevy::sprite::collide_aabb::{Collision, collide};
use rand::random;

const GAP_BETWEEN_PADDLE_WALL: f32 = 30.0;
const PADDLE_SCALE_X: f32 = 20.0;
const PADDLE_SCALE_Y: f32 = 100.0;
const PADDLE_SPEED: f32 = 450.0;
const WW: f32 = 1280.0;
const WH: f32 = 720.0;
const BALL_SPEED: f32 = 400.0;
const BALL_RADIUS: f32 = 10.0;


#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    direction: Vec2,
}

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Resource)]
struct Scoreboard1 {
    score: u32,
}

impl Default for Scoreboard1 {
    fn default() -> Scoreboard1 {
        Scoreboard1 {
            score: 0,
        }
    }
}

#[derive(Resource)]
struct Scoreboard2 {
    score: u32,
}

impl Default for Scoreboard2 {
    fn default() -> Scoreboard2 {
        Scoreboard2 {
            score: 0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".to_string(),
                    resizable: false,
                    resolution: (1280.0, 720.0).into(),
                    position: WindowPosition::Automatic,
                    //mode: WindowMode::SizedFullscreen,
                    ..default()
                }),
                ..default()
            })
        )
        .init_resource::<Scoreboard1>()
        .init_resource::<Scoreboard2>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_player1_paddle,
            move_player2_paddle,
        ))
        .add_systems(Update, ball_movement) 
        .add_systems(Update, ball_paddle1_collision)
        .add_systems(Update, scoring_system)
        .add_systems(Update, ball_paddle2_collision)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let celadon = Color::hex("#B8D8BA").unwrap();
    let paddle_color = Color::hex("#fcddbc").unwrap();
    let ball_color = Color::hex("#69585f").unwrap();

    //clear color to celadon
    commands.insert_resource(ClearColor(celadon));  
    
    //spawn the camera entity
    commands.spawn(Camera2dBundle{
        ..default()
    });


    //spawn player1 paddle 
    let player1_x = -((WW/2.0) - GAP_BETWEEN_PADDLE_WALL);
    //let player1_y: f32;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: paddle_color,
                custom_size: Some(Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)),
                ..default()
        },
        transform: Transform::from_xyz(player1_x, 0.0, 0.0),
        ..default()
        },
        Paddle,
        Player1
    ));

    //spawn player2 paddle
    let player2_x = (WW/2.0) - GAP_BETWEEN_PADDLE_WALL;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: paddle_color,
                custom_size: Some(Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)),
                ..default()
            },
            transform: Transform::from_xyz(player2_x, 0.0, 0.0),
            ..default()
        },
        Paddle,
        Player2
    ));

    //spawn ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(ball_color)),
            transform: Transform::from_xyz(0.0, 0.0 ,0.0),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
        }
    ));

    //player 1 score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle { 
                        ..default()
                    }
            ),          
            transform: Transform {
                translation: Vec3::new(-100.0, WH/2.0 - 20.0, 0.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..default()
            },  
            ..default()
        },
        Player1
    ));

    //player2 score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle { 
                        ..default()
                    }
            ),          
            transform: Transform {
                translation: Vec3::new(100.0, WH/2.0 - 20.0, 0.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..default()
            },  
            ..default()
        },
        Player2
    ));

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2{
                    x: 2.0,
                    y: WH,
                }),
                ..default()
            },
            ..default()
        }
    );
}

//move player1 paddle
fn move_player1_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut player1: Query<&mut Transform, (With<Player1>, With<Paddle>)>,
    time: Res<Time>,
) {
    let mut direction1 = 0.0;
    let mut paddle_p1 = player1.single_mut();

    //move player1 paddle
    if keyboard_input.pressed(KeyCode::W) {
        direction1 += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction1 -= 1.0;
    }

    let paddle1_new_pos = paddle_p1.translation.y + direction1 * PADDLE_SPEED * time.delta_seconds();
    paddle_p1.translation.y = paddle1_new_pos;

    if paddle_p1.translation.y <= -(WH/2.0 - PADDLE_SCALE_Y/2.0) {
        paddle_p1.translation.y = -(WH/2.0 - PADDLE_SCALE_Y/2.0);
    }
    else if paddle_p1.translation.y >= WH/2.0 - PADDLE_SCALE_Y/2.0 {
        paddle_p1.translation.y = WH/2.0 - PADDLE_SCALE_Y/2.0;
    }
}

//move player2 paddle
fn move_player2_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut player2: Query<&mut Transform, (With<Player2>, With<Paddle>)>,
    time: Res<Time>
) {
    let mut direction2 = 0.0;
    let mut paddle_p2 = player2.single_mut();
    if keyboard_input.pressed(KeyCode::Up) {
        direction2 += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction2 -= 1.0;
    }

    let paddle2_new_pos = paddle_p2.translation.y + direction2 * PADDLE_SPEED * time.delta_seconds();
    paddle_p2.translation.y = paddle2_new_pos;

    if paddle_p2.translation.y <= -(WH/2.0 - PADDLE_SCALE_Y/2.0) {
        paddle_p2.translation.y = -(WH/2.0 - PADDLE_SCALE_Y/2.0);
    }
    else if paddle_p2.translation.y >= WH/2.0 - PADDLE_SCALE_Y/2.0{
        paddle_p2.translation.y = WH/2.0 - PADDLE_SCALE_Y/2.0;
    }
}

fn ball_movement(
    mut pong: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
) {
        for (mut transform, mut ball) in pong.iter_mut() {
            let direction = Vec3::new(ball.direction.x, ball.direction.y, 0.0);
            transform.translation += direction * BALL_SPEED * time.delta_seconds();
            
            //collision ball and wall
            if transform.translation.y <= -(WH/2.0 - BALL_RADIUS/2.0) {
                ball.direction.y = -ball.direction.y;
            }
            if transform.translation.y >= (WH/2.0 - BALL_RADIUS/2.0) {
                ball.direction.y = -ball.direction.y;
            }       
        }
    
}

fn ball_paddle1_collision(
    paddle1: Query<&Transform, (With<Player1>, With<Paddle>)>,
    mut ball: Query<(&Transform, &mut Ball)>,
) {
    let player1 = paddle1.single();

    for (ball_transform, mut ball) in ball.iter_mut() {  
        let collision = collide(
            ball_transform.translation,
            Vec2::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0),
            player1.translation,
            Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)            
        );

        let collision = match collision {
            Some(collision) => collision,
            None => continue,
        };

        let (reflect_x, reflect_y) = match collision {
            Collision::Left => (ball.direction.x > 0.0, false),
            Collision::Right => (ball.direction.x < 0.0, false),
            Collision::Top => (false, ball.direction.y < 0.0),
            Collision::Bottom => (false, ball.direction.y > 0.0),
            Collision::Inside => continue,
        };

        if reflect_x {
            ball.direction.x = -ball.direction.x;
        }

        if reflect_y {
            ball.direction.y = -ball.direction.y;
        }
    }
}

fn ball_paddle2_collision(
    player: Query<&Transform, (With<Player2>, With<Paddle>)>,
    mut ball: Query<(&Transform, &mut Ball)>
) {
    let player2 = player.single();

    for(ball_transform, mut _ball) in ball.iter_mut() {
        let collision = collide(
            ball_transform.translation,
            Vec2::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0),
            player2.translation,
            Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)            
        );

        let collision = match collision {
            Some(collision) => collision,
            None => continue,
        };

        let (reflect_x, reflect_y) = match collision {
            Collision::Left => (_ball.direction.x > 0.0, false),
            Collision::Right => (_ball.direction.x < 0.0, false),
            Collision::Top => (false, _ball.direction.y < 0.0),
            Collision::Bottom => (false, _ball.direction.y > 0.0),
            Collision::Inside => continue,
        };

        if reflect_x {
            _ball.direction.x = -_ball.direction.x;
        }

        if reflect_y {
            _ball.direction.y = -_ball.direction.y;
        }
    }
}


fn scoring_system(
    mut score1: ResMut<Scoreboard1>,
    mut score2: ResMut<Scoreboard2>,
    mut ball: Query<&mut Transform, With<Ball>>,
    mut text_p1: Query<&mut Text, (With<Player1>, Without<Player2>)>,
    mut text_p2: Query<&mut Text, (With<Player2>, Without<Player1>)>
) {
    let mut ball = ball.single_mut();
    let mut txt1 = text_p1.single_mut();
    let mut txt2 = text_p2.single_mut();

    if ball.translation.x >= WW/2.0 - BALL_RADIUS {
        score1.score += 1;
        txt1.sections[0].value = score1.score.to_string();
        println!("{}", score1.score);
        ball.translation = Vec3::new(0.0, 0.0, 0.0);
    }
    if ball.translation.x <= -(WW/2.0 - BALL_RADIUS) {
        score2.score += 1;
        txt2.sections[0].value = score2.score.to_string();
        println!("{}", score2.score);
        ball.translation = Vec3::new(0.0, 0.0, 0.0);
    }
}