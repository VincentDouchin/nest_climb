use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct FeatherBundle {
    pickup: Pickup,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Feather;
