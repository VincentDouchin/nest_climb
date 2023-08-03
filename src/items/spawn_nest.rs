use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Flag {
    pub next_level: usize,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct NestBundle {
    #[ldtk_entity]
    pub flag: Flag,
    #[bundle]
    entity_collider_bundle: EntityColliderBundle,
}
impl LdtkEntity for Flag {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let level_index = entity_instance.get_int_field("Next_Level").unwrap();
        return Flag {
            next_level: level_index.clone() as usize,
        };
    }
}
pub fn spawn_nest(
    assets: Res<MyAssets>,
    mut query: Query<(Entity, &mut Transform), Added<Flag>>,
    mut commands: Commands,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y -= 8.0;
        commands
            .entity(entity)
            .insert(AnimatedSpriteBundle::new(assets.nest.clone()));
    }
}

pub fn level_transition(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Run);
}

pub fn move_to_next_level(
    nest_query: Query<(Entity, &Flag)>,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (nest_entity, nest) in nest_query.iter() {
        for player_entity in player_query.iter() {
            if let Some(contact) = rapier_context.intersection_pair(nest_entity, player_entity) {
                if contact {
                    commands.insert_resource(LevelSelection::Index(nest.next_level));
                    next_state.set(GameState::LevelTransition)
                }
            }
        }
    }
}
