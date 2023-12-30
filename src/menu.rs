use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
//use bevy_kira_audio::prelude::*;

use crate::derivables::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .insert_resource(Language::English)
            .insert_resource(Volume(7))
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Startup, setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle{
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::Fixed{width: VIEW_SIZE.x, height: VIEW_SIZE.y},
			..default()
		},
		..default()
		}, OnSplashScreen
    ));
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(0.0, 0.0, 100.0),
		texture: asset_server.load("splash_screen.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(VIEW_SIZE.x, VIEW_SIZE.y)),
			..default()
		},
		..default()
		},
		OnSplashScreen,
	));
    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Game);
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}