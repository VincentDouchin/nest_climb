use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;

#[derive(Component, Clone, Default)]
pub struct Trampoline;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct TrampolineBundle {
    trampoline: Trampoline,
    ghost_platform: GhostPlatform,
}

pub fn spawn_trampoline(
    assets: Res<MyAssets>,
    mut commands: Commands,
    query: Query<Entity, Added<Trampoline>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            AnimatedSpriteBundle {
                texture_atlas_handle: assets.trampoline.clone(),
                timer: AnimationTimer::stopped(),
                ..default()
            },
            Collider::cuboid(16.0, 8.0),
        ));
    }
}

pub fn bounce_on_trampoline(
    mut trampoline_query: Query<(Entity, &mut AnimationTimer), With<Trampoline>>,
    mut player_query: Query<(&TnuaProximitySensor, &mut ExternalImpulse), With<Player>>,
) {
    for (proximity_sensor, mut impulse) in player_query.iter_mut() {
        for (trampoline_entity, mut trampoline_animation_timer) in trampoline_query.iter_mut() {
            let is_touching_top_of_trampoline = proximity_sensor
                .output
                .clone()
                .map_or(false, |output| output.entity == trampoline_entity);
            if is_touching_top_of_trampoline {
                trampoline_animation_timer.state = AnimationTimerState::Once;
                impulse.impulse = Vec2::Y * 5.0;
            }
        }
    }
}
