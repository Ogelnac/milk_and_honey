use bevy::prelude::*;

// Constants
pub const VIEW_SIZE: Vec2 = Vec2::new(1600.0, 900.0);

// States
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Splash,
    Game,
}

// Resources
#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct MousePosition {
    pub pos: Vec3,
}

#[derive(Resource)]
pub struct Level(pub usize);

#[derive(Resource)]
pub struct Played(pub bool);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    English,
    //Spanish,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

// Components
#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Player {
    pub direction: f32,
}

#[derive(Component)]
pub struct OnSplashScreen;