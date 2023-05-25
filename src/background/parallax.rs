use bevy::prelude::*;

use bevy_parallax::*;

use crate::*;

pub fn target_camera_for_parallax(
    mut commands: Commands,
    camera_query: Query<Entity, Added<MainCamera>>,
) {
    if let Ok(entity) = camera_query.get_single() {
        commands.entity(entity).insert(ParallaxCameraComponent);
    }
}
pub fn add_parallax_layers(
    mut commands: Commands,
    mut parallax: ResMut<ParallaxResource>,
    assets: Res<MyAssets>,
    images: Res<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let get_layer_data = |handles: Vec<&Handle<Image>>| -> Vec<LayerData> {
        return handles
            .iter()
            .map(|handle| {
                let dimensions = images.get(handle).unwrap().size();
                return LayerData {
                    speed: LayerSpeed::Bidirectional(0.5, 1.0),
                    texture: handle.clone().clone(),
                    tile_size: dimensions,
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 1.0,
                    ..default()
                };
            })
            .collect();
    };

    parallax.layer_data = get_layer_data(vec![
        &assets.parallax_back,
        &assets.parallax_far,
        &assets.parallax_middle,
    ]);
    parallax.create_layers(&mut commands, &mut texture_atlases);
}

pub fn parallax_plugin(app: &mut App) {
    app.add_plugin(ParallaxPlugin);
    app.init_resource::<ParallaxResource>();
    app.add_system(target_camera_for_parallax);
    app.add_system(add_parallax_layers.in_schedule(OnEnter(GameState::InitRun)));
}
