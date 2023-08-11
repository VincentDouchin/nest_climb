use crate::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct MovementControl {
    pub desired_velocity: Vec2,
    pub acceleration: f32,
    pub deceleration: f32,
    pub air_acceleration: f32,
    pub air_deceleration: f32,
    pub can_move: bool,
    pub turn_speed: f32,
    pub speed: f32,
    pub grounded: bool,
    pub desired_jump: bool,
    pub coyote_timer: Timer,
    pub jump_buffer_timer: Timer,
    pub jump_height: f32,
    pub pressing_jump: bool,
    pub jumping: bool,
    pub upward_multiplier: f32,
    pub downward_multiplier: f32,
    pub jump_cut_off: f32,
    pub max_fall_speed: f32,
}

impl MovementControl {
    pub fn tick(&mut self, time: &Res<Time>) {
        self.jump_buffer_timer.tick(time.delta());
        self.coyote_timer.tick(time.delta());
        if self.jump_buffer_timer.finished() {
            self.desired_jump = false;
        }
    }
}

impl Default for MovementControl {
    fn default() -> Self {
        MovementControl {
            speed: 100.0,
            desired_velocity: Vec2::ZERO,
            can_move: true,
            acceleration: 200.0,
            deceleration: 100.0,
            air_acceleration: 300.0,
            air_deceleration: 000.0,
            turn_speed: 500.0,
            grounded: true,
            desired_jump: false,
            coyote_timer: Timer::from_seconds(0.2, TimerMode::Once),
            jump_buffer_timer: Timer::from_seconds(0.2, TimerMode::Once),
            jump_height: 200.0,
            pressing_jump: false,
            jumping: false,
            upward_multiplier: 1.0,
            downward_multiplier: 3.0,
            jump_cut_off: 3.0,
            max_fall_speed: 100.0,
        }
    }
}

pub fn move_player_system(
    mut query: Query<(&ActionState<PlayerAction>, &mut MovementControl)>,
    time: Res<Time>,
) {
    for (actions, mut controls) in query.iter_mut() {
        controls.tick(&time);
        let mut direction = 0.0;
        if actions.pressed(PlayerAction::MoveLeft) {
            direction += -1.0;
        }
        if actions.pressed(PlayerAction::MoveRight) {
            direction += 1.0;
        }
        if actions.just_pressed(PlayerAction::Jump) {
            controls.desired_jump = true;
            controls.pressing_jump = true;
            controls.jump_buffer_timer.reset()
        }
        if actions.just_released(PlayerAction::Jump) {
            controls.pressing_jump = false
        }
        controls.desired_velocity.x = direction * controls.speed;
    }
}

fn move_towards(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else {
        current + (target - current).signum() * max_delta
    }
}

pub fn apply_movement(mut query: Query<(&mut MovementControl, &mut Velocity)>, time: Res<Time>) {
    for (mut controls, mut velocity) in query.iter_mut() {
        let mut max_speed_change: f32 = if controls.grounded {
            controls.deceleration
        } else {
            controls.air_deceleration
        };
        if controls.desired_velocity.x.abs() > 0.1 {
            max_speed_change = if controls.desired_velocity.x.signum() != velocity.linvel.x.signum()
            {
                controls.turn_speed
            } else if controls.grounded {
                controls.acceleration
            } else {
                controls.air_acceleration
            };
        }
        println!("{}", controls.grounded);
        if controls.desired_jump && controls.grounded {
            controls.desired_jump = false;
            controls.jumping = true;
            velocity.linvel.y = controls.jump_height
        }
        velocity.linvel.x = move_towards(
            velocity.linvel.x,
            controls.desired_velocity.x,
            max_speed_change * time.delta_seconds(),
        );
    }
}

fn update_gravity_scale(mut query: Query<(&mut GravityScale, &Velocity, &MovementControl)>) {
    for (mut gravity, velocity, controls) in query.iter_mut() {
        let mut grav_multiplier = 1.0;
        if velocity.linvel.y > 0.1 {
            if controls.grounded {
                grav_multiplier = 1.0
            } else if controls.pressing_jump && controls.jumping {
                grav_multiplier = controls.upward_multiplier
            } else {
                grav_multiplier = controls.jump_cut_off
            }
        } else if velocity.linvel.y < -0.1 {
            if controls.grounded {
                grav_multiplier = 1.0
            } else {
                grav_multiplier = controls.downward_multiplier
            }
        }

        gravity.0 = grav_multiplier;
    }
}
#[derive(Component)]
pub struct GroundSensor {
    pub target: Entity,
    pub output: Vec<Entity>,
}
fn add_ground_sensor(
    mut commands: Commands,
    query: Query<(Entity, &Collider), Added<MovementControl>>,
) {
    for (entity, collider) in query.iter() {
        let size = collider.as_cuboid().unwrap().half_extents();
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0,
                    -size.y - 0.6,
                    0.0,
                ))),
                RigidBody::Fixed,
                Collider::cuboid(size.x - 1.0, 0.5),
                GroundSensor {
                    target: entity,
                    output: Vec::new(),
                },
                Sensor,
            ));
        });
    }
}
fn update_ground_sensor(
    mut player_query: Query<(Entity, &mut MovementControl)>,
    sensor_query: Query<(Entity, &GroundSensor)>,
    rapier_context: Res<RapierContext>,
) {
    for (player_entity, mut controls) in player_query.iter_mut() {
        for (sensor_entity, sensor) in sensor_query.iter() {
            if sensor.target == player_entity {
                controls.grounded = rapier_context
                    .intersections_with(sensor_entity)
                    .any(|(contact, _, _)| contact != player_entity);
            }
        }
    }
}
// fn update_ground_sensor(
//     mut control_query: Query<(&mut MovementControl, &Collider, &Transform)>,
//     rapier_context: Res<RapierContext>,
// ) {
//     for (mut control, collider, transform) in control_query.iter_mut() {
//         let size = collider.as_cuboid().unwrap().half_extents();
//         let is_grounded = rapier_context
//             .intersection_with_shape(
//                 Vec2::new(0.0, -size.y - 0.5)
//                     + Vec2::new(transform.translation.x, transform.translation.y),
//                 0.0,
//                 &Collider::cuboid(size.x - 1.0, 0.5),
//                 QueryFilter::new().exclude_sensors(),
//             )
//             .is_some();
//         if control.grounded && !is_grounded {
//             control.coyote_timer.reset();
//         }
//         if is_grounded {
//             control.jumping = false
//         }
//         control.grounded = is_grounded
//     }
// }
pub fn movement_plugin(app: &mut App) {
    app.add_systems((
        apply_movement,
        update_ground_sensor,
        add_ground_sensor,
        update_gravity_scale,
    ));
}
