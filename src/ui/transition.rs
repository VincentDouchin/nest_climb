use crate::{GameState, MyAssets};
use bevy::prelude::*;

#[derive(Component)]
pub struct TransitionContainer {
    timer: Timer,
}
impl Default for TransitionContainer {
    fn default() -> Self {
        TransitionContainer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub fn spawn_transition_container(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Global(100),
            ..default()
        },
        TransitionContainer::default(),
    ));
}

// pub struct AltasImageLens {
//     len: usize,
// }

// impl Lens<UiAtlasImage> for AltasImageLens {
//     fn lerp(&mut self, target: &mut UiAtlasImage, ratio: f32) {
//         target.index = (self.len as f32 * ratio).floor() as usize
//     }
// }

// impl AltasImageLens {
//     pub fn new(handle: &Handle<TextureAtlas>, atlases: &Res<Assets<TextureAtlas>>) -> Self {
//         let maybe_atlas = atlases.get(handle);
//         AltasImageLens {
//             len: maybe_atlas.map_or(0, |atlas| atlas.len()),
//         }
//     }
// }

pub fn spawn_transition(
    mut query: Query<(Entity, &mut TransitionContainer)>,
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    assets: Res<MyAssets>,
    mut commands: Commands,
) {
    for (entity, mut transition_container) in query.iter_mut() {
        transition_container.timer.tick(time.delta());
        if transition_container.timer.just_finished() {
            commands.spawn(AtlasImageBundle {
                // style: Style {
                //     position: UiRect {
                //         left: Val::Px(0.0),
                //         right: Val::Auto,
                //         top: Val::Px(0.0),
                //         bottom: Val::Auto,
                //     },
                //     position_type: PositionType::Absolute,
                //     size: Size::all(Val::Px(64.0)),
                //     ..default()
                // },
                // atlas_image: UiAtlasImage::new(assets.transition.clone(), 5),
                ..default()
            });
            // let tween = Tween::new(
            //     EaseFunction::QuadraticInOut,
            //     Duration::from_secs(3),
            //     AltasImageLens::new(&assets.transition, &atlases),
            // )
            // .with_repeat_count(RepeatCount::Finite(1));
            // commands.spawn(SpriteSheetBundle {
            //     texture_atlas: assets.transition.clone(),
            //     sprite: TextureAtlasSprite::new(5),
            //     transform: Transform::from_translation(Vec3::new(200.0, 200.0, 1.0)),
            //     ..default()
            // });
            // commands.entity(entity).with_children(|container| {
            //     let id = container
            //         .spawn(AtlasImageBundle {
            //             style: Style {
            //                 size: Size::all(Val::Percent(10.0)),
            //                 ..default()
            //             },
            //             atlas_image: UiAtlasImage::new(assets.transition.clone(), 5),
            //             ..default()
            //         })
            //         .id();
            //     dbg!(id);
            // });
        }
    }
}

pub fn transition_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Run), spawn_transition_container)
        .add_systems(
            Update,
            spawn_transition.run_if(not(in_state(GameState::AssetLoading))),
        );
}
