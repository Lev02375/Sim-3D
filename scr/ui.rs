use bevy::prelude::*;
use crate::Score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_score);
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Счёт: 0"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.5)),
            ScoreText,
        ));
    });
    
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("WASD — движение | Мышь — обзор | ЛКМ — стрельба | ESC — курсор"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.0 = format!("Счёт: {}", score.0);
        }
    }
}
