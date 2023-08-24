use crate::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;

pub fn run_timer_plugin(app: &mut App) {
    app.add_systems(Update, update_health_timer.run_if(in_state(GameState::Run)));
}

pub fn update_health_timer(mut health_query: Query<&mut Health>, time: Res<Time>) {
    for mut health in health_query.iter_mut() {
        if let Some(timer) = health.timer.as_mut() {
            timer.tick(time.delta());
        }
    }
}

pub fn player_enemy_interaction(
    mut player_query: Query<
        (Entity, &mut Health, &TnuaProximitySensor),
        (With<Player>, Without<DamagePlayer>),
    >,
    mut enemy_query: Query<
        (Entity, Option<&mut Health>, Option<&SoftHead>),
        (With<DamagePlayer>, Without<Player>),
    >,
    rapier_context: Res<RapierContext>,
) {
    for (player, mut player_health, tnua_proximity_output) in player_query.iter_mut() {
        for (enemy, maybe_enemy_health, maybe_soft_head) in enemy_query.iter_mut() {
            if rapier_context.contact_pair(player, enemy).is_some() {
                let is_touching_top_of_enemy = tnua_proximity_output
                    .output
                    .clone()
                    .map_or(false, |output| output.entity == enemy);

                if is_touching_top_of_enemy && maybe_soft_head.is_some() {
                    if let Some(mut enemy_health) = maybe_enemy_health {
                        enemy_health.take_damage(1)
                    }
                } else {
                    player_health.take_damage(1)
                }
            } else if rapier_context.intersection_pair(player, enemy).is_some() {
                player_health.take_damage(1)
            }
        }
    }
}
