use crate::*;
use bevy_easings::*;
use rand::random;

#[derive(Component)]
pub struct Cloud(f32);

pub fn spawn_title(
    mut commands: Commands,
    assets: Res<MyAssets>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
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
    commands.spawn((
        ImageBundle {
            image: UiImage::new(assets.title_text.clone()),
            z_index: ZIndex::Global(1),
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
pub fn title_plugin(app: &mut App) {
    app.add_system(spawn_title.in_schedule(OnEnter(GameState::Start)))
        .add_system(move_clouds);
}
