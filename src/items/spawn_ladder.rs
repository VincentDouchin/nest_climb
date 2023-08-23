use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::TnuaToggle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Component)]
pub struct Climber {
    pub is_climbing: bool,
    pub can_climb: bool,
    pub climbing_speed: f32,
}
impl Climber {
    pub fn new(speed: f32) -> Self {
        Climber {
            is_climbing: false,
            can_climb: false,
            climbing_speed: speed,
        }
    }
}

pub fn ignore_gravity_if_climbing(
    mut query: Query<(&Climber, &mut GravityScale, Option<&mut TnuaToggle>), Changed<Climber>>,
) {
    for (climber, mut gravity_scale, mut maybe_toggle) in &mut query {
        gravity_scale.0 = if climber.is_climbing { 0.0 } else { 1.0 };
        if let Some(toggle) = maybe_toggle.as_deref_mut() {
            if climber.is_climbing {
                *toggle = TnuaToggle::Disabled
            } else {
                *toggle = TnuaToggle::Enabled
            }
        }
    }
}

pub fn detect_can_climb(
    mut climber_query: Query<(Entity, &mut Climber)>,
    climbable_query: Query<Entity, With<Climbable>>,
    rapier_context: Res<RapierContext>,
) {
    for (climber_entity, mut climber) in climber_query.iter_mut() {
        climber.can_climb = climbable_query.iter().any(|climbable_entity| {
            return rapier_context
                .contact_pair(climber_entity, climbable_entity)
                .is_some();
        });
        if !climber.can_climb && climber.is_climbing {
            climber.is_climbing = false
        }
    }
}
