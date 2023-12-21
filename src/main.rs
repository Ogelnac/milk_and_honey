// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::f32::consts::PI;
use std::time::Duration; 

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    let default_plugins = DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window {
                title: "Milk and Honey".to_string(),
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent:true,
                ..default()
            }),
            ..default()
        })
    ;


    App::new()
        .add_plugins((default_plugins, AudioPlugin))
        .insert_resource(MousePosition{pos:Vec3::ZERO})
        .insert_resource(Level(0))
        .insert_resource(Played(false))
        .add_systems(Startup, (setup, spawn_environment))
        .add_systems(Update, (setup_scene_once_loaded, mouse_position, move_player, move_camera))
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Ground;

#[derive(Component)]
pub struct Player {
    pub direction: f32,
}

#[derive(Resource)]
struct MousePosition {
    pub pos: Vec3,
}

#[derive(Resource)]
pub struct Level(pub usize);

#[derive(Resource)]
pub struct Played(pub bool);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("character/protag_0_0_0.glb#Animation0"),
        asset_server.load("character/protag_0_0_0.glb#Animation1"),
        asset_server.load("character/protag_0_0_0.glb#Animation2"),
        asset_server.load("character/protag_0_0_0.glb#Animation3"),
    ]));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection {
                aspect_ratio: 16.0/9.0,
                ..default()
                }.into(),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        brightness: 0.6,
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        point_light: PointLight {
            intensity: 10000000.0,
            range: 500.0,
            color: Color::rgb(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_environment(
    mut commands: Commands,
    assets: Res<AssetServer>,
    level: Res<Level>,
) {
    let environment_scene = assets.load("environment/level_0.glb#Scene0");
    let ground_scene = assets.load("environment/interactor_level_0.glb#Scene0");
    let player = assets.load("character/protag_0_0_0.glb#Scene0");
    let rock = assets.load("environment/hint_rock.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: environment_scene,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn((SceneBundle {
        scene: ground_scene,
        transform: Transform::from_xyz(0.0, -0.09, 0.0),
        ..Default::default()
    },
    Ground,
    ));

    commands.spawn(SceneBundle {
        scene: rock,
        transform: Transform::from_xyz(-5.0, 0.0, 12.0)
        .with_scale(Vec3::splat(7.5))
        .with_rotation(Quat::from_rotation_y(PI / 10.0)),
        ..Default::default()
    });

    let mut start_direction = 0.0;

    match level.0 {
        0 => start_direction = -PI / 2.0,
        _ => {},
    }

    commands.spawn((SceneBundle {
        scene: player,
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
        .with_scale(Vec3::splat(1.5))
        .with_rotation(Quat::from_rotation_y(start_direction)),
        ..default()
    },
    Player {
        direction: start_direction,
    },
    ));
}

fn mouse_position(
    ground_query: Query<&GlobalTransform, With<Ground>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let ground = ground_query.single();
    let (camera, camera_transform) = camera_query.single();
    
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(ground.translation(), ground.up()) else {
        return;
    };

    let position = ray.get_point(distance);

    if buttons.just_released(MouseButton::Left) {
        if position.z.abs() < 15.0 {
            if position.x < 100.0 && position.x > 0.0 {
                mouse_position.pos = position;
                println!("{}", mouse_position.pos);
            }
        }
    }
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn move_camera(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (_, mut camera_transform) = camera_query.single_mut();

    if keyboard_input.pressed(KeyCode::D) {
        camera_transform.translation += Vec3::new(0.2, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.translation -= Vec3::new(0.2, 0.0, 0.0);
    }
}

fn move_player(
    mut player_query: Query<(&mut Transform, With<Player>)>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut current_animation: Local<usize>,
    mouse_position: ResMut<MousePosition>,
    animations: Res<Animations>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut played: ResMut<Played>,
) {
    let (mut player_transform, _) = player_query.single_mut();
    let distance = (mouse_position.pos - player_transform.translation).length();

    player_transform.look_at(mouse_position.pos, Vec3::Y);
    
    for mut player in &mut animation_players {
        if distance > 0.5 {
            if !played.0 {
                audio.play(asset_server.load("sfx/bubbles.ogg"));

                played.0 = true;
            }

            let direction = (mouse_position.pos - player_transform.translation).normalize();
            player_transform.translation += direction * 0.2;
    
            *current_animation = 3;
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        } else {
            played.0 = false;

            *current_animation = 2;
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
    }
}