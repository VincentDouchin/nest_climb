use bevy::prelude::*;

use crate::{GameState, MenuButton, MyAssets, NineSlice, StateUi};
use bevy_ui_navigation::prelude::*;

pub fn spawn_start_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(16.0)),
                    size: Size::new(Val::Px(256.0), Val::Px(64.0)),
                    ..default()
                },
                // background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            MenuButton::GoToGameState(GameState::LevelSelect),
            Focusable::default(),
            StateUi(GameState::Start),
            NineSlice {
                image_handle: assets.button_big.clone(),
                margins: Vec4::splat(16.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start Game",
                TextStyle {
                    font: assets.default_font.clone(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}
