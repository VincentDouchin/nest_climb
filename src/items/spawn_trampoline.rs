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
            AnimatedSpriteBundle {
                texture_atlas_handle: assets.trampoline.clone(),
                timer: AnimationTimer::stopped(),
                ..default()
            },
            Collider::cuboid(16.0, 8.0),
        ));
    }
}

#[derive(Component)]
pub struct BouncingOnTrampoline(pub Option<f32>);

pub fn bounce_on_trampoline(
    mut trampoline_query: Query<(Entity, &mut AnimationTimer), With<Trampoline>>,
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
        for (trampoline_entity, mut trampoline_animation_timer) in trampoline_query.iter_mut() {
            let is_touching_top_of_trampoline = proximity_sensor
                .output
                .clone()
                .map_or(false, |output| output.entity == trampoline_entity);

            if is_touching_top_of_trampoline {
                trampoline_animation_timer.state = AnimationTimerState::Once;
                bouncing.0 = Some(if actions.pressed(PlayerAction::Jump) {
                    bouncing.0.map_or(1.0, |amount| amount + 1.0).max(4.0)
                } else {
                    1.0
                });
                controls.jump = bouncing.0;
            } else if bouncing.0.is_some() {
                controls.jump = None
            }
            if output.jumping_velocity.is_none() && !is_touching_top_of_trampoline {
                bouncing.0 = None
            }
        }
    }
}
