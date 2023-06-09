use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Health {
    pub last_health: u32,
    pub took_damage: bool,
    pub current_health: u32,
    pub max_health: u32,
    pub timer: Option<Timer>,
}
#[derive(Component, Clone, Default)]
pub struct Ghost;
impl Health {
    pub fn new(max_health: u32) -> Self {
        return Health {
            current_health: max_health,
            max_health,
            timer: None,
            last_health: max_health,
            took_damage: false,
        };
    }
    pub fn take_damage(&mut self, amount: u32) {
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
pub fn detect_health_changed(mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        health.took_damage = health.last_health > health.current_health;
        health.last_health = health.current_health.clone();
    }
}
pub fn kill_entity(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &Health,
            &Transform,
            Option<&DeathAnimation>,
            Option<&Enemy>,
            Option<&Player>,
        ),
        Changed<Health>,
    >,
    player_buttons_query: Query<Entity, With<PlayerButtons>>,

    mut score: ResMut<Score>,
) {
    for (entity, health, transform, maybe_death_animation, maybe_enemy, maybe_player) in
        query.iter()
    {
        if health.current_health <= 0 {
            if let Some(death_animation) = maybe_death_animation {
                if maybe_player.is_some() {
                    for entity in player_buttons_query.iter() {
                        commands.entity(entity).despawn_recursive()
                    }
                }
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: death_animation.0.clone(),
                        transform: transform.clone(),
                        ..default()
                    },
                    Ghost,
                    DespawnWhenAnimationFinished,
                    AnimationTimer::default(),
                ));
            }
            commands.entity(entity).despawn_recursive();

            if maybe_enemy.is_some() {
                score.enemies_killed += 1;
            }
        }
    }
}

pub fn move_ghost(mut query: Query<&mut Transform, With<Ghost>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y += 10.0 * time.delta_seconds()
    }
}
