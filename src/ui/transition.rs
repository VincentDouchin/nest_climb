use crate::*;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
pub struct Transition {
    timer: Timer,
    current_row: u32,
    max_rows: u32,
}
impl Transition {
    fn new(max_rows: u32) -> Self {
        Transition {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_row: 0,
            max_rows,
        }
    }
    fn finished(&self) -> bool {
        self.max_rows == self.current_row
    }
}

#[derive(Resource)]
pub struct LevelToSet(pub usize);

pub fn set_level_on_transition(
    mut commands: Commands,
    level_to_set_option: Option<Res<LevelToSet>>,
) {
    if let Some(level_to_set) = level_to_set_option {
        commands.insert_resource(LevelSelection::Index(level_to_set.0));
    }
}

#[derive(Component)]
pub struct TransitionContainer;

pub fn spawn_transition_container(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<MyAssets>,
) {
    if let Ok(window) = window_query.get_single() {
        let rows = (window.height() / (window.width() * 0.1)).ceil() as u32;
        commands.insert_resource(Transition::new(rows));
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_wrap: FlexWrap::Wrap,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    z_index: ZIndex::Global(100),
                    ..default()
                },
                TransitionContainer,
            ))
            .with_children(|container| {
                for row in 0..rows {
                    for _ in 1..11 {
                        container.spawn((
                            AtlasImageBundle {
                                style: Style {
                                    width: Val::Percent(10.0),
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                texture_atlas: assets.transition.clone(),
                                ..default()
                            },
                            TransitionElement {
                                timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                                row,
                            },
                        ));
                    }
                }
            });
    }
}

pub fn despawn_transition_container(
    mut commands: Commands,
    query: Query<Entity, With<TransitionContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}

#[derive(Component)]
pub struct TransitionElement {
    pub timer: Timer,
    pub row: u32,
}

pub fn transition_in(
    mut query: Query<(&mut UiTextureAtlasImage, &mut TransitionElement)>,
    mut transition: ResMut<Transition>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    assets: Res<MyAssets>,
    time: Res<Time>,
) {
    transition.timer.tick(time.delta());

    if let Some(atlas) = texture_atlases.get(&assets.transition) {
        let max_index = atlas.len() - 1;
        if transition.timer.just_finished() {
            if transition.current_row < transition.max_rows {
                transition.current_row += 1;
            }
            if transition.finished()
                && query
                    .iter()
                    .all(|(atlas_image, _)| atlas_image.index == max_index)
            {
                next_transition_state.set(TransitionState::Out);
                next_game_state.set(GameState::Run);
            }
        }
        for (mut atlas_image, mut transition_element) in query.iter_mut() {
            transition_element.timer.tick(time.delta());
            if transition_element.timer.just_finished()
                && transition_element.row <= transition.current_row
            {
                atlas_image.index = (atlas_image.index + 1).min(atlas.len() - 1);
            }
        }
    }
}
pub fn transition_out(
    mut query: Query<(&mut UiTextureAtlasImage, &mut TransitionElement)>,
    mut transition: ResMut<Transition>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    time: Res<Time>,
) {
    transition.timer.tick(time.delta());

    if transition.timer.just_finished() {
        if transition.current_row > 0 {
            transition.current_row -= 1;
        }
        if transition.current_row == 0
            && query.iter().all(|(atlas_image, _)| atlas_image.index == 0)
        {
            next_transition_state.set(TransitionState::None);
        }
    }

    for (mut atlas_image, mut transition_element) in query.iter_mut() {
        transition_element.timer.tick(time.delta());
        if transition_element.timer.just_finished()
            && transition_element.row >= transition.current_row
            && atlas_image.index > 0
        {
            atlas_image.index = atlas_image.index - 1
        }
    }
}

pub fn transition_plugin(app: &mut App) {
    app.add_state::<TransitionState>()
        .add_systems(OnEnter(TransitionState::In), spawn_transition_container)
        .add_systems(
            OnExit(TransitionState::In),
            (despawn_map, set_level_on_transition),
        )
        .add_systems(OnEnter(TransitionState::Out), spawn_map)
        .add_systems(OnExit(TransitionState::Out), despawn_transition_container)
        .add_systems(Update, transition_in.run_if(in_state(TransitionState::In)))
        .add_systems(
            Update,
            transition_out.run_if(in_state(TransitionState::Out)),
        );
}
