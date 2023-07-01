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
}

pub fn spawn_trampoline(
    assets: Res<MyAssets>,
    mut commands: Commands,
    query: Query<Entity, Added<Trampoline>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            AnimatedSpriteBundle::new(assets.trampoline.clone()),
            Collider::cuboid(8.0, 4.0),
        ));
    }
}

pub fn bounce_on_trampoline(
    trampoline_query: Query<Entity, With<Trampoline>>,
    mut player_query: Query<
        (
            Entity,
            &TnuaProximitySensor,
            &mut TnuaPlatformerControls,
            &Velocity,
            &TnuaPlatformerConfig,
        ),
        With<Player>,
    >,
    rapier_context: Res<RapierContext>,
) {
    for (player, proximity_sensor, mut controls, velocity, config) in player_query.iter_mut() {
        for trampoline in trampoline_query.iter() {
            if rapier_context.contact_pair(player, trampoline).is_some() {
                let is_touching_top_of_trampoline = proximity_sensor
                    .output
                    .clone()
                    .map_or(false, |output| output.entity == trampoline);
                if is_touching_top_of_trampoline {
                    controls.jump =
                        Some((0.5 * velocity.linvel.y / -config.full_jump_height).clamp(1.0, 3.0));
                }
            }
        }
    }
}
