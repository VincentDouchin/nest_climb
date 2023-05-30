use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Health {
    pub current_health: u32,
    pub max_health: u32,
    pub timer: Option<Timer>,
}
impl Health {
    pub fn new(max_health: u32) -> Self {
        return Health {
            current_health: max_health,
            max_health,
            timer: None,
        };
    }
    pub fn update_health(&mut self, amount: u32) {
        if let Some(timer) = self.timer.as_mut() {
            if timer.finished() {
                self.timer = None;
            }
        }
        if self.timer.is_none() {
            if self.current_health > 0 {
                self.current_health -= amount;
                self.timer = Some(Timer::from_seconds(1.0, TimerMode::Once));
            }
        }
    }
}

impl LdtkEntity for Health {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let max_health = entity_instance
            .get_int_field("health")
            .expect("Enemy should have health")
            .clone()
            .try_into()
            .unwrap();
        return Health::new(max_health);
    }
}

pub fn kill_entity(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Enemy>), Changed<Health>>,
    mut score: ResMut<Score>,
) {
    for (entity, health, is_enemy) in query.iter() {
        if health.current_health <= 0 {
            commands.entity(entity).despawn_recursive();
            if is_enemy.is_some() {
                score.enemies_killed += 1
            }
        }
    }
}
