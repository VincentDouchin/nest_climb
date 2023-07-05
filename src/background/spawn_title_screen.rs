use crate::*;
use bevy_easings::*;
pub fn spawn_title(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        ImageBundle {
            image: UiImage::new(assets.title_nest.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        },
        StateUi(GameState::Start),
    ));
    commands.spawn((
        ImageBundle {
            image: UiImage::new(assets.title_text.clone()),

            ..default()
        },
        Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Auto),
            position: UiRect::top(Val::Percent(-100.0)),
            ..default()
        }
        .ease_to(
            Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                position: UiRect::top(Val::Percent(0.0)),
                ..default()
            },
            EaseFunction::BounceOut,
            EasingType::Once {
                duration: std::time::Duration::from_secs(3),
            },
        ),
        StateUi(GameState::Start),
    ));
    [
        assets.cloud_1.clone(),
        assets.cloud_2.clone(),
        assets.cloud_3.clone(),
    ]
    .iter()
    .enumerate()
    .for_each(|(index, cloud_handle)| {
        commands.spawn(ImageBundle {
            image: UiImage::new(cloud_handle.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(index as f32 / 3.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        });
    });
}

pub fn title_plugin(app: &mut App) {
    app.add_system(spawn_title.in_schedule(OnEnter(GameState::Start)));
}
