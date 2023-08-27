use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tnua::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct FeatherBundle {
    pickup: Pickup,
    feather: Feather,
    #[bundle()]
    entity_collider_bundle: EntityColliderBundle,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Feather;

#[derive(Component, Copy, Clone, PartialEq, Debug, Default)]
pub struct Feathered;

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
    mut player_query: Query<(Entity, &mut ExternalForce), With<Player>>,
    feather_query: Query<Entity, With<Feather>>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (player_entity, mut external_force) in player_query.iter_mut() {
        for feather_entity in feather_query.iter() {
            if rapier_context
                .intersection_pair(player_entity, feather_entity)
                .is_some()
            {
                external_force.force += Vec2::Y * 250.0;
                commands.entity(player_entity).insert(Feathered);
            }
        }
    }
}

pub fn reset_feathered_gravity(
    mut query: Query<(Entity, &TnuaProximitySensor, &mut ExternalForce), With<Feathered>>,
    mut commands: Commands,
) {
    for (entity, output, mut external_force) in query.iter_mut() {
        if output.output.is_some() {
            external_force.force -= Vec2::Y * 250.0;
            commands.entity(entity).remove::<Feathered>();
        }
    }
}

pub fn feather_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_feather, touch_feather, reset_feathered_gravity).run_if(in_state(GameState::Run)),
    );
}
