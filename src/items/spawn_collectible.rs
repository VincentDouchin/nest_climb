use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Pickup;

#[derive(Clone, Bundle)]
pub struct EntityColliderBundle {
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
}

impl Default for EntityColliderBundle {
    fn default() -> Self {
        EntityColliderBundle {
            body: RigidBody::Fixed,
            collider: Collider::cuboid(8.0, 8.0),
            sensor: Sensor,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CollectibleBundle {
    #[bundle]
    entity_collider_bundle: EntityColliderBundle,
    pickup: Pickup,
    #[ldtk_entity]
    pub collectible_type: CollectibleType,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum CollectibleType {
    #[default]
    Coin = 10,
    Gem = 20,
    BottleCap = 50,
}

impl LdtkEntity for CollectibleType {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        return match entity_instance
            .get_enum_field("Collectible")
            .unwrap()
            .as_str()
        {
            "Gem" => CollectibleType::Gem,
            "BottleCap" => CollectibleType::BottleCap,
            _ => CollectibleType::Coin,
        };
    }
}

pub fn spawn_collectibles(
    mut commands: Commands,
    query: Query<(Entity, &CollectibleType), Added<CollectibleType>>,
    assets: Res<MyAssets>,
) {
    for (entity, collectible_type) in query.iter() {
        let handle = match collectible_type {
            _ => &assets.coin,
        };
        commands
            .entity(entity)
            .insert(AnimatedSpriteBundle::new(handle.clone()));
    }
}

pub fn collect_item(
    mut commands: Commands,
    collectible_query: Query<(Entity, Option<&CollectibleType>), With<Pickup>>,
    player_query: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut score: ResMut<Score>,
) {
    for (collectible_entity, maybe_collectible) in collectible_query.iter() {
        for player_entity in player_query.iter() {
            if rapier_context
                .intersection_pair(collectible_entity, player_entity)
                .is_some()
            {
                if let Some(collectible_value) = maybe_collectible {
                    score.collectibles += collectible_value.clone() as u32;
                }

                commands.entity(collectible_entity).despawn_recursive()
            }
        }
    }
}