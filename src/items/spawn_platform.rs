use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Platform;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlatformBundle {
    platform: Platform,
    #[ldtk_entity]
    pub patrol: Patrol,
}

pub fn spawn_platforms(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), Added<Platform>>,
    assets: Res<MyAssets>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(1.0);
        commands.entity(entity).insert((
            AnimatedSpriteBundle::new(assets.platform.clone()),
            Collider::cuboid(24.0, 8.0),
            Velocity::default(),
            RigidBody::Dynamic,
            KinematicCharacterController::default(),
            LockedAxes::ROTATION_LOCKED,
            Ccd::default(),
            SolverGroups {
                // Or pick some other configuration - as long as it excludes the character
                // that needs to jump or fall through this platform.
                memberships: Group::empty(),
                filters: Group::empty(),
            },
            TnuaGhostPlatform,
        ));
    }
}

fn apply_tnua_fall_through_controls(
    mut query: Query<(&mut TnuaProximitySensor, &TnuaGhostSensor)>,
) {
    for (mut proximity_sensor, ghost_sensor) in query.iter_mut() {
        for ghost_platform in ghost_sensor.iter() {
            if 12.0 <= ghost_platform.proximity {
                proximity_sensor.output = Some(ghost_platform.clone());
                break;
            }
        }
    }
}
// #[derive(Component, Default)]
// pub struct TnuaSimpleFallThroughPlatformsHelper {
//     currently_falling_through: HashSet<Entity>,
// }

// impl TnuaSimpleFallThroughPlatformsHelper {
//     /// Get an handle for operating the helper.
//     ///
//     /// The `min_proximity` argument is the minimal distance from the origin of the cast ray/shape
//     /// (usually the center of the character) to the platform. If the distance to the platform is
//     /// below that, the helper will assume that the character only jumped halfway through it, not
//     /// high enough to stand on it.
//     pub fn with<'a>(
//         &'a mut self,
//         proximity_sensor: &'a mut TnuaProximitySensor,
//         ghost_sensor: &'a TnuaGhostSensor,
//         min_proximity: f32,
//     ) -> TnuaHandleForSimpleFallThroughPlatformsHelper<'a> {
//         TnuaHandleForSimpleFallThroughPlatformsHelper {
//             parent: self,
//             proximity_sensor,
//             ghost_sensor,
//             min_proximity,
//         }
//     }
// }
// pub struct TnuaHandleForSimpleFallThroughPlatformsHelper<'a> {
//     parent: &'a mut TnuaSimpleFallThroughPlatformsHelper,
//     proximity_sensor: &'a mut TnuaProximitySensor,
//     ghost_sensor: &'a TnuaGhostSensor,
//     min_proximity: f32,
// }

// impl TnuaHandleForSimpleFallThroughPlatformsHelper<'_> {
//     /// Call this method to make the character stand on the platform (if there is any)
//     pub fn dont_fall(&mut self) {
//         let mut already_falling_through_not_yet_seen =
//             self.parent.currently_falling_through.clone();
//         for ghost_platform in self.ghost_sensor.iter() {
//             if self.min_proximity <= ghost_platform.proximity
//                 && !already_falling_through_not_yet_seen.remove(&ghost_platform.entity)
//             {
//                 self.proximity_sensor.output = Some(ghost_platform.clone());
//                 break;
//             }
//         }
//         self.parent
//             .currently_falling_through
//             .retain(|entity| !already_falling_through_not_yet_seen.contains(entity));
//     }

//     /// Call this method to make the character drop through the platform.
//     ///
//     /// The character will fall through the first layer of ghost platforms detected since the last
//     /// time it was called with `just_pressed` being `true`. This means that:
//     ///
//     /// * To let the player fall through all the platforms by simply holding the button, call this
//     /// with `just_pressed = true` as long as the button is held.
//     /// * To let the player fall through one layer of platforms at a time, forcing them to release
//     /// and press again for each layer, pass `just_pressed = true` only when the button really is
//     /// just pressed.
//     ///
//     /// Returns `true` if actually dropping through a platform, to help determining if the
//     /// character should be crouching (since these buttons are usually the same)
//     pub fn try_falling(&mut self, just_pressed: bool) -> bool {
//         if !just_pressed && !self.parent.currently_falling_through.is_empty() {
//             for ghost_platform in self.ghost_sensor.iter() {
//                 if self.min_proximity <= ghost_platform.proximity
//                     && !self
//                         .parent
//                         .currently_falling_through
//                         .contains(&ghost_platform.entity)
//                 {
//                     self.proximity_sensor.output = Some(ghost_platform.clone());
//                     return true;
//                 }
//             }
//             return true;
//         }
//         self.parent.currently_falling_through.clear();
//         for ghost_platform in self.ghost_sensor.iter() {
//             if self.min_proximity <= ghost_platform.proximity {
//                 self.parent
//                     .currently_falling_through
//                     .insert(ghost_platform.entity);
//             }
//         }
//         !self.parent.currently_falling_through.is_empty()
//     }
// }

// pub fn platform_plugin(app: &mut App) {
//     app.add_system(apply_tnua_fall_through_controls.in_set(TnuaUserControlsSystemSet));
// }
