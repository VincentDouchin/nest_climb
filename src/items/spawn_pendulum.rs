use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::*,
    utils::{ldtk_pixel_coords_to_translation, ldtk_pixel_coords_to_translation_pivoted},
};
use bevy_rapier2d::prelude::*;

use crate::*;

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct Pendulum {
    pub origin: Vec2,
    pub limit: f32,
    pub forward: bool,
    pub speed: f32,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PendulumBundle {
    #[ldtk_entity]
    pendulum: Pendulum,
    damage_player: DamagePlayer,
}
impl LdtkEntity for Pendulum {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let pixel_coords = (entity_instance
            .get_point_field("origin")
            .expect("the origin of the pendulum")
            .as_vec2()
            + Vec2::new(0.5, 0.5))
            * Vec2::splat(layer_instance.grid_size as f32);

        let origin = ldtk_pixel_coords_to_translation_pivoted(
            pixel_coords.as_ivec2(),
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        );
        let limit_coords = (entity_instance
            .get_point_field("limit")
            .expect("the origin of the pendulum")
            .as_vec2()
            + Vec2::new(0.5, 0.5))
            * Vec2::splat(layer_instance.grid_size as f32);
        let limit_transform = ldtk_pixel_coords_to_translation(
            limit_coords.as_ivec2(),
            layer_instance.c_hei * layer_instance.grid_size,
        );
        let limit = (limit_transform.x - origin.x).abs();
        let speed = entity_instance
            .get_float_field("speed")
            .expect("pendulum speed")
            .clone();
        return Pendulum {
            origin,
            limit,
            speed,
            forward: true,
        };
    }
}

pub fn spawn_pendulum(
    mut query: Query<(Entity, &Pendulum, &mut Transform), Added<Pendulum>>,
    assets: Res<MyAssets>,
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
) {
    if let Ok(level_entity) = level_query.get_single() {
        for (entity, pendulum, mut transform) in query.iter_mut() {
            transform.scale = Vec3::splat(1.0);
            let joint = RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, pendulum.origin.y - transform.translation.y))
                .local_anchor2(Vec2::new(0.0, 0.0));
            let parent = commands
                .spawn((
                    RigidBody::Fixed,
                    ImpulseJoint::new(entity, joint),
                    GlobalTransform::default(),
                    Transform::from_xyz(pendulum.origin.x, pendulum.origin.y, 0.0),
                ))
                .id();
            commands.entity(level_entity).add_child(parent);

            commands.entity(entity).insert((
                AnimatedSpriteBundle::new(assets.spikyball.clone()),
                RigidBody::Dynamic,
                Velocity::default(),
                Collider::ball(16.0),
            ));
        }
    }
}

pub fn move_pendulum(mut query: Query<(&mut Velocity, &mut Pendulum, &Transform)>) {
    for (mut velocity, mut pendulum, transform) in query.iter_mut() {
        let direction = if pendulum.forward { 1.0 } else { -1.0 };
        velocity.angvel = pendulum.speed * direction;
        if pendulum.forward && transform.translation.x >= pendulum.origin.x + pendulum.limit {
            pendulum.forward = false
        }
        if !pendulum.forward && transform.translation.x <= pendulum.origin.x - pendulum.limit {
            pendulum.forward = true
        }
    }
}
