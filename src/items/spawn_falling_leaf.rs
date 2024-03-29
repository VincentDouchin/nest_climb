use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::*};

#[derive(Bundle, LdtkEntity)]
pub struct FallingLeafBundle {
    #[ldtk_entity]
    falling_leaf: FallingLeaf,
}
impl LdtkEntity for FallingLeaf {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let speed = entity_instance.get_int_field("speed").ok().unwrap();
        let origin = ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        )
        .y;
        let limit_point = (entity_instance
            .get_point_field("limit")
            .ok()
            .unwrap()
            .as_vec2()
            + Vec2::new(0.5, 0.5))
            * Vec2::splat(layer_instance.grid_size as f32);

        let limit = ldtk_pixel_coords_to_translation_pivoted(
            limit_point.as_ivec2(),
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        )
        .y;
        FallingLeaf {
            timer: Timer::from_seconds(speed.clone() as f32, TimerMode::Repeating),
            limit: origin - limit,
            enabled: true,
        }
    }
}

#[derive(Component, Debug)]
pub struct FallingLeaf {
    pub timer: Timer,
    pub limit: f32,
    pub enabled: bool,
}

#[derive(Component)]
pub struct Leaf(pub f32);

pub fn spawn_leafs(
    mut leaf_generators_query: Query<(Entity, &mut FallingLeaf)>,
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<MyAssets>,
) {
    for (entity, mut falling_leaf) in leaf_generators_query.iter_mut() {
        if falling_leaf.enabled {
            falling_leaf.timer.tick(time.delta());
            if falling_leaf.timer.finished() {
                commands.entity(entity).with_children(|generator| {
                    generator.spawn((
                        assets.trampoline.clone(),
                        TextureAtlasSprite::default(),
                        TransformBundle::from_transform(Transform::from_translation(Vec3::Z)),
                        VisibilityBundle::default(),
                        Collider::cuboid(16.0, 4.0),
                        // GhostPlatform,
                        RigidBody::KinematicVelocityBased,
                        Velocity {
                            linvel: Vec2::new(0.0, -10.0),
                            ..default()
                        },
                        Leaf(falling_leaf.limit),
                    ));
                });
            }
        }
    }
}

pub fn despawn_fallen_leafs(query: Query<(Entity, &Transform, &Leaf)>, mut commands: Commands) {
    for (entity, transform, leaf) in query.iter() {
        if transform.translation.y < -leaf.0 {
            commands.entity(entity).despawn_recursive()
        }
    }
}
