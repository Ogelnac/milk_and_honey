// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

// Modules
mod derivables;
mod menu;
mod game_setup;

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
        .add_plugins((
            default_plugins, 
            AudioPlugin,
            menu::MenuPlugin,
            game_setup::GameSetupPlugin,
        ))
        .run();
}