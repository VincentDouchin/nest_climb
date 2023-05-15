mod animation {
    mod animation_components;
    pub use self::animation_components::*;
    mod animation_system;
    pub use self::animation_system::*;
}
pub use animation::*;

mod map {
    mod ground;
    pub use ground::*;
    mod map;
    pub use map::*;
    mod walls;
    pub use walls::*;
}
pub use map::*;

mod global {
    mod assets;
    pub use self::assets::*;
    mod camera;
    pub use self::camera::*;
    mod states;
    pub use self::states::*;
    mod physics;
    pub use self::physics::*;
    mod init_libs;
    pub use init_libs::*;
}
pub use global::*;

mod player {
    mod player_controls;
    pub use player_controls::*;
    mod move_player;
    pub use self::move_player::*;
    mod player_entity;
    pub use self::player_entity::*;
}
pub use player::*;

mod debug;
pub use debug::*;

mod ui {
    mod health;
    pub use health::*;
}
pub use ui::*;

mod health {
    mod health_component;
    pub use health_component::*;
}
pub use health::*;
