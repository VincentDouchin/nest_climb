mod components {
    mod animation_timer;
    pub use self::animation_timer::*;
}
pub use components::*;

mod entities {
    mod player;
    pub use self::player::*;
}
pub use entities::*;

mod systems {
    mod animation_system;
    pub use self::animation_system::*;
}
pub use systems::*;

mod global {
    mod assets;
    pub use self::assets::*;
    mod camera;
    pub use self::camera::*;
    mod states;
    pub use self::states::*;
}
pub use global::*;
