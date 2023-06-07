use bevy::prelude::*;

use crate::{GameState, MyAssets, StateUi};

#[derive(Component)]
pub struct StartButton;

pub fn spawn_start_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            },
            StartButton,
            StateUi(GameState::Start),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start Game",
                TextStyle {
                    font: assets.default_font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub fn start_game(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if interaction == &Interaction::Clicked {
            next_state.set(GameState::LevelSelect)
        }
    }
}
