use bevy::prelude::*;

#[derive(Component)]
pub struct HeartContainer;

pub fn spawn_run_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        },
        HeartContainer,
    ));
}
