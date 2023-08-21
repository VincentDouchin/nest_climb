use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

#[derive(Component)]
pub struct Level;
#[derive(Clone, Bundle)]
pub struct EntityColliderBundle {
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
}

impl Default for EntityColliderBundle {
    fn default() -> Self {
        EntityColliderBundle {
            body: RigidBody::Fixed,
            collider: Collider::cuboid(8.0, 8.0),
            sensor: Sensor,
        }
    }
}

#[derive(Copy, Clone, Component)]
pub struct TilesetId(pub Option<i32>);

impl LdtkIntCell for TilesetId {
    fn bundle_int_cell(_int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self {
        TilesetId(layer_instance.tileset_def_uid)
    }
}

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct AnimatedTileBundle {
    #[ldtk_int_cell]
    tileset_id: TilesetId,
}

#[derive(Component)]
pub struct AnimatedTiles {
    pub tile_indexes: Vec<i32>,
    pub timer: Timer,
}
impl AnimatedTiles {
    pub fn new(tiles: Vec<i32>) -> Self {
        AnimatedTiles {
            tile_indexes: tiles,
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        }
    }
}

pub fn spawn_animated_tiles(
    query: Query<(Entity, &TileEnumTags, &TilesetId), Without<AnimatedTiles>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    current_level: Res<CurrentLevel>,
    mut commands: Commands,
) {
    for (entity, tags, tileset_id) in query.iter() {
        if let Some(id) = tileset_id.0 {
            if let Some(handle) = &current_level.file {
                if let Some(level) = ldtk_assets.get(handle) {
                    for tileset in level.project.defs.tilesets.iter() {
                        if tileset.uid == id {
                            for enum_tag in tileset.enum_tags.iter() {
                                for tag in tags.tags.iter() {
                                    if tag == &enum_tag.enum_value_id {
                                        commands
                                            .entity(entity)
                                            .insert(AnimatedTiles::new(enum_tag.tile_ids.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub fn animate_tiles(
    mut query: Query<(&mut TileTextureIndex, &mut AnimatedTiles)>,
    time: Res<Time>,
) {
    for (mut tile_index, mut animated_tiles) in query.iter_mut() {
        animated_tiles.timer.tick(time.delta());
        if animated_tiles.timer.finished() {
            let index = animated_tiles
                .tile_indexes
                .iter()
                .position(|&r| r == tile_index.0 as i32)
                .unwrap();
            tile_index.0 =
                animated_tiles.tile_indexes[(index + 1) % animated_tiles.tile_indexes.len()] as u32;
        }
    }
}
pub fn map_plugin(app: &mut App) {
    app.insert_resource(LevelSelection::Index(0))
        .init_resource::<CurrentLevel>()
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_int_cell::<WallBundle>(4)
        .register_ldtk_int_cell::<WallBundle>(5)
        .register_ldtk_int_cell::<WallBundle>(8)
        .register_ldtk_int_cell::<WallBundle>(8)
        .register_ldtk_int_cell::<AnimatedTileBundle>(9)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_entity::<CollectibleBundle>("Collectible")
        .register_ldtk_entity::<NestBundle>("Nest")
        .register_ldtk_entity::<PlatformBundle>("Platform")
        .register_ldtk_entity::<SawbladeBundle>("Sawblade")
        .register_ldtk_entity::<PendulumBundle>("Pendulum")
        .register_ldtk_entity::<HeartBundle>("Heart")
        .register_ldtk_entity::<TrampolineBundle>("Trampoline")
        .register_ldtk_entity::<FallingLeafBundle>("FallingLeaf")
        .register_ldtk_entity::<DeadLeafBundle>("DeadLeaf")
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::No,
            ..Default::default()
        })
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Run)))
        .add_system(despawn_map.in_schedule(OnExit(GameState::Run)))
        .add_systems((spawn_animated_tiles, animate_tiles))
        .add_systems(
            (
                spawn_player,
                spawn_walls,
                spawn_enemy,
                spawn_collectibles,
                spawn_ghost_platforms,
                spawn_platforms,
                spawn_sawblade,
                spawn_pendulum,
                spawn_nest,
                spawn_heart,
                spawn_trampoline,
                spawn_leafs,
                spawn_dead_leaves,
                despawn_fallen_leafs,
                disappear_platforms,
            )
                .in_set(OnUpdate(GameState::Run)),
        );
}

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub file: Option<Handle<LdtkAsset>>,
}

pub fn spawn_map(mut commands: Commands, maybe_current_level: Res<CurrentLevel>) {
    if let Some(current_level) = &maybe_current_level.file {
        commands.spawn((
            LdtkWorldBundle {
                ldtk_handle: current_level.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Level,
        ));
    }
}
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Level>>) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
