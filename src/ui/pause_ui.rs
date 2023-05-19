use crate::*;
use bevy::prelude::*;

pub fn spawn_pause_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        TextBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text::from_section(
                "Game paused",
                TextStyle {
                    font: assets.default_font.clone(),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),

            ..default()
        },
        StateUi {
            state: GameState::Pause,
        },
    ));
}
