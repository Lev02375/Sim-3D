use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;

mod player;
mod world;
mod shooting;
mod ui;

use player::PlayerPlugin;
use world::WorldPlugin;
use shooting::ShootingPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Neon Labyrinth".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((PlayerPlugin, WorldPlugin, ShootingPlugin, UiPlugin))
        .insert_resource(Score(0))
        .add_systems(Startup, setup_cursor)
        .add_systems(Update, toggle_cursor)
        .run();
}

#[derive(Resource)]
pub struct Score(pub i32);

fn setup_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn toggle_cursor(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = window_query.get_single_mut() {
            let is_locked = window.cursor_options.grab_mode == CursorGrabMode::Locked;
            window.cursor_options.grab_mode = if is_locked {
                CursorGrabMode::None
            } else {
                CursorGrabMode::Locked
            };
            window.cursor_options.visible = is_locked;
        }
    }
      }
