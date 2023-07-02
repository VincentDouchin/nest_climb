use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;

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

#[derive(Component)]
pub struct BouncingOnTrampoline(pub Option<f32>);

pub fn bounce_on_trampoline(
    trampoline_query: Query<Entity, With<Trampoline>>,
    mut player_query: Query<
        (
            &TnuaProximitySensor,
            &mut TnuaPlatformerControls,
            &TnuaPlatformerAnimatingOutput,
            &mut BouncingOnTrampoline,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    for (proximity_sensor, mut controls, output, mut bouncing, actions) in player_query.iter_mut() {
        let is_touching_top_of_trampoline =
            proximity_sensor.output.clone().map_or(false, |output| {
                trampoline_query
                    .iter()
                    .any(|entity| entity == output.entity)
            });

        if is_touching_top_of_trampoline {
            bouncing.0 = Some(if actions.pressed(PlayerAction::Jump) {
                bouncing.0.map_or(1.0, |amount| amount + 1.0).max(4.0)
            } else {
                1.0
            });
            if actions.pressed(PlayerAction::Jump) {}

            controls.jump = bouncing.0;
        } else if bouncing.0.is_some() {
            controls.jump = None
        }
        if output.jumping_velocity.is_none() && !is_touching_top_of_trampoline {
            bouncing.0 = None
        }
    }
}
