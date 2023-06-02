use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::*;

pub fn run_timer_plugin(app: &mut App) {
    app.add_system(update_health_timer.in_set(OnUpdate(GameState::Run)));
}

pub fn update_health_timer(mut health_query: Query<&mut Health>, time: Res<Time>) {
    for mut health in health_query.iter_mut() {
        if let Some(timer) = health.timer.as_mut() {
            timer.tick(time.delta());
        }
    }
}

pub fn player_enemy_interaction(
    mut player_query: Query<(Entity, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>,
    rapier_context: Res<RapierContext>,
) {
    for (player, mut player_health) in player_query.iter_mut() {
        for (enemy, mut enemy_health) in enemy_query.iter_mut() {
            if let Some(contact_pair) = rapier_context.contact_pair(player, enemy) {
                if contact_pair.has_any_active_contacts() {
                    if let Some((contact, _)) = contact_pair.find_deepest_contact() {
                        if contact.normal() == Vec2::new(0.0, -1.0) {
                            enemy_health.update_health(1);
                        } else {
                            player_health.update_health(1)
                        }
                    }
                }
            }
        }
    }
}
