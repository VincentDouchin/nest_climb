use crate::*;
use bevy::{prelude::*, utils::*};
use bevy_easings::*;
use bevy_tweening::{lens::*, *};
use bevy_ui_navigation::prelude::*;
use rand::*;

#[derive(Component)]
pub struct Cloud(f32);

struct UiStretchLens {
    start: f32,
    end: f32,
}

impl Lens<Style> for UiStretchLens {
    fn lerp(&mut self, target: &mut Style, ratio: f32) {
        target.size.width = Val::Percent(&self.start + (&self.end - &self.start) * ratio);
    }
}

pub fn spawn_start_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Percent(40.0),
                        bottom: Val::Auto,
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(16.0)),
                    size: Size::new(Val::Px(256.0), Val::Px(64.0)),
                    ..default()
                },
                z_index: ZIndex::Global(3),
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
    commands.spawn((
        ImageBundle {
            image: UiImage::new(assets.title_nest.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        },
        StateUi(GameState::Start),
    ));
    // ! TEXT
    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs(3),
        UiPositionLens {
            start: UiRect::bottom(Val::Percent(100.0)),
            end: UiRect::all(Val::Percent(0.0)),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1));

    commands.spawn((
        ImageBundle {
            image: UiImage::new(assets.title_text.clone()),
            z_index: ZIndex::Global(1),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Auto),
                position: UiRect::top(Val::Percent(0.0)),
                ..default()
            },
            ..default()
        },
        Animator::new(tween),
        StateUi(GameState::Start),
    ));
    if let Ok(projection) = camera_query.get_single() {
        [
            assets.cloud_1.clone(),
            assets.cloud_2.clone(),
            assets.cloud_3.clone(),
        ]
        .iter()
        .for_each(|cloud_handle| {
            commands.spawn((
                SpriteBundle {
                    texture: cloud_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        projection.area.max.x
                            * random::<f32>()
                            * if random::<bool>() { 1.0 } else { -1.0 },
                        projection.area.max.y * random::<f32>(),
                        1.0,
                    )),
                    ..default()
                },
                Cloud((0.5 + random::<f32>()) * 20.0),
                StateUi(GameState::Start),
            ));
        });
    }
}

pub fn move_clouds(
    images: Res<Assets<Image>>,
    mut cloud_query: Query<(&mut Transform, &Cloud, &Handle<Image>)>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(projection) = camera_query.get_single() {
        for (mut cloud_transform, cloud_speed, cloud_handle) in cloud_query.iter_mut() {
            if let Some(cloud_image) = images.get(cloud_handle) {
                cloud_transform.translation.x -= cloud_speed.0 * time.delta_seconds();
                if cloud_transform.translation.x
                    < projection.area.min.x - cloud_image.size().x / 2.0
                {
                    cloud_transform.translation.x =
                        projection.area.max.x + cloud_image.size().x + 100.0 * random::<f32>();
                }
            }
        }
    }
}
