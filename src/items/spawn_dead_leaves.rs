use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tnua::TnuaProximitySensor;

enum DeadLeafState {
    Idle,
    Touched,
    Falling,
    Despawned,
}

#[derive(Component)]
pub struct DeadLeaf {
    existing_leaf: Option<Entity>,
    touch_timer: Timer,
    despawn_timer: Timer,
    respawn_timer: Timer,
    state: DeadLeafState,
    initialized: bool,
}
impl Default for DeadLeaf {
    fn default() -> Self {
        DeadLeaf {
            existing_leaf: None,
            touch_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            despawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            respawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            state: DeadLeafState::Idle,
            initialized: false,
        }
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct DeadLeafBundle {
    dead_leaf: DeadLeaf,
    body: RigidBody,
    #[bundle]
    visibility_bundle: VisibilityBundle,
}

pub fn spawn_dead_leaves(
    mut spawner_query: Query<(Entity, &mut DeadLeaf)>,
    player_query: Query<&TnuaProximitySensor, With<Player>>,
    mut commands: Commands,
    assets: Res<MyAssets>,
    time: Res<Time>,
) {
    for (dead_leaf_entity, mut dead_leaf) in spawner_query.iter_mut() {
        if let Ok(tnua_sensor) = player_query.get_single() {
            match dead_leaf.state {
                DeadLeafState::Idle => {}
                DeadLeafState::Touched => {
                    dead_leaf.touch_timer.tick(time.delta());
                }
                DeadLeafState::Falling => {
                    dead_leaf.despawn_timer.tick(time.delta());
                }
                DeadLeafState::Despawned => {
                    dead_leaf.respawn_timer.tick(time.delta());
                }
            }
            if let Some(entity) = dead_leaf.existing_leaf {
                if tnua_sensor
                    .output
                    .clone()
                    .map_or(false, |output| output.entity == entity)
                {
                    dead_leaf.state = DeadLeafState::Touched
                }

                if dead_leaf.touch_timer.finished() {
                    dead_leaf.touch_timer.reset();
                    dead_leaf.state = DeadLeafState::Falling;
                    commands
                        .entity(entity)
                        .insert(Velocity::linear(Vec2::new(0.0, -30.0)));
                }
                if dead_leaf.despawn_timer.finished() {
                    dead_leaf.despawn_timer.reset();
                    dead_leaf.state = DeadLeafState::Despawned;
                    commands.entity(entity).despawn_recursive();
                    dead_leaf.existing_leaf = None;
                }
            }
            if dead_leaf.respawn_timer.finished() || !dead_leaf.initialized {
                dead_leaf.initialized = true;
                dead_leaf.respawn_timer.reset();
                dead_leaf.state = DeadLeafState::Idle;
                let leaf = commands
                    .spawn((
                        SpriteBundle {
                            texture: assets.deadleaf.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        },
                        RigidBody::KinematicVelocityBased,
                        Collider::cuboid(24.0, 4.0),
                    ))
                    .id();

                commands.entity(dead_leaf_entity).add_child(leaf);
                dead_leaf.existing_leaf = Some(leaf);
            }
        }
    }
}
