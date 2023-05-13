mod components {
    mod animation_components;
    pub use self::animation_components::*;
    mod player_controls;
    pub use self::player_controls::*;
}
pub use components::*;

mod entities {
    mod player;
    pub use self::player::*;
    mod ground;
    pub use self::ground::*;
}
pub use entities::*;

mod systems {
    mod animation_system;
    pub use self::animation_system::*;
    mod move_player_system;
    pub use self::move_player_system::*;
}
pub use systems::*;

mod global {
    mod assets;
    pub use self::assets::*;
    mod camera;
    pub use self::camera::*;
    mod states;
    pub use self::states::*;
    mod physics;
    pub use self::physics::*;
}
pub use global::*;
