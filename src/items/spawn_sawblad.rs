use crate::*;
pub use bevy::prelude::*;
pub use bevy_ecs_ldtk::prelude::*;
pub use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Sawblade;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SawbladeBundle {
    damage_player: DamagePlayer,
    sawblade: Sawblade,
    #[ldtk_entity]
    pub patrol: Patrol,
}

pub fn spawn_sawblade(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut query: Query<(Entity, &Patrol, &mut Transform), Added<Sawblade>>,
) {
    for (entity, patrol, mut transform) in query.iter_mut() {
        if patrol.points.len() > 1 {
            commands
                .entity(entity)
                .insert((RigidBody::Dynamic, KinematicCharacterController::default()))
        } else {
            commands.entity(entity).insert(RigidBody::Fixed)
        };
        transform.scale = Vec3::splat(1.0);
        commands.entity(entity).insert((
            AnimatedSpriteBundle::new(assets.sawblade.clone()),
            Collider::ball(28.0),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Sensor,
        ));
    }
}
