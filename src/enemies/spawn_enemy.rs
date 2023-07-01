use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct SoftHead;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct DamagePlayer;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
    damage_player: DamagePlayer,
    #[ldtk_entity]
    pub patrol: Patrol,
    #[ldtk_entity]
    pub health: Health,
    soft_head: SoftHead,
}

pub fn spawn_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, Added<Enemy>>,
    assets: Res<MyAssets>,
) {
    for entity in enemy_query.iter() {
        let bundle = (
            Collider::cuboid(8.0, 8.0),
            Velocity::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            AnimatedSpriteBundle::new(assets.bushly_idle.clone()),
        );
        commands.entity(entity).insert(bundle);
    }
}
