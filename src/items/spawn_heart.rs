use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct HeartBundle {
    #[bundle()]
    entity_collider_bundle: EntityColliderBundle,
    pickup: Pickup,
    #[bundle()]
    heart: Heart,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Heart;

pub fn spawn_heart(
    query: Query<Entity, Added<Heart>>,
    assets: Res<MyAssets>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(AnimatedSpriteBundle::new(assets.heart.clone()));
    }
}
