use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tnua::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct FeatherBundle {
    pickup: Pickup,
    feather: Feather,
    #[bundle]
    entity_collider_bundle: EntityColliderBundle,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Feather;

#[derive(Component, Copy, Clone, PartialEq, Debug, Default)]
pub struct Feathered(pub f32);

pub fn spawn_feather(
    mut commands: Commands,
    query: Query<Entity, Added<Feather>>,
    assets: Res<MyAssets>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(AnimatedSpriteBundle::new(assets.feather.clone()));
    }
}

pub fn touch_feather(
    mut player_query: Query<(Entity, &mut TnuaPlatformerConfig), With<Player>>,
    feather_query: Query<Entity, With<Feather>>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (player_entity, mut config) in player_query.iter_mut() {
        for feather_entity in feather_query.iter() {
            if rapier_context
                .intersection_pair(player_entity, feather_entity)
                .is_some()
            {
                commands
                    .entity(player_entity)
                    .insert(Feathered(config.jump_fall_extra_gravity));
                config.jump_fall_extra_gravity = -250.0
            }
        }
    }
}

pub fn reset_feathered_gravity(
    mut query: Query<(
        Entity,
        &TnuaProximitySensor,
        &mut TnuaPlatformerConfig,
        &Feathered,
    )>,
    mut commands: Commands,
) {
    for (entity, output, mut config, feather) in query.iter_mut() {
        if output.output.is_some() {
            config.jump_fall_extra_gravity = feather.0;
            commands.entity(entity).remove::<Feathered>();
        }
    }
}

pub fn feather_plugin(app: &mut App) {
    app.add_systems(
        (spawn_feather, touch_feather, reset_feathered_gravity).in_set(OnUpdate(GameState::Run)),
    );
}
