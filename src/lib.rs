mod animation {
    mod animation_components;
    pub use self::animation_components::*;
    mod animation_system;
    pub use self::animation_system::*;
}
pub use animation::*;

mod map {
    mod map;
    pub use self::map::*;
    mod walls;
    pub use self::walls::*;
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
    pub use self::init_libs::*;
}
pub use global::*;

mod player {
    mod player_controls;
    pub use self::player_controls::*;
    mod move_player;
    pub use self::move_player::*;
    mod spawn_player;
    pub use self::spawn_player::*;
}
pub use player::*;

mod debug;
pub use debug::*;

mod ui {
    mod health;
    pub use self::health::*;
    mod run_ui;
    pub use self::run_ui::*;
    mod pause_ui;
    pub use self::pause_ui::*;
    mod ui;
    pub use self::ui::*;
    mod start_ui;
    pub use self::start_ui::*;
    mod level_select_ui;
    pub use self::level_select_ui::*;
}
pub use ui::*;

mod health {
    mod health_component;
    pub use self::health_component::*;
}
pub use health::*;

mod enemies {
    mod spawn_enemy;
    pub use self::spawn_enemy::*;
}
pub use enemies::*;

mod background {
    mod parallax;
    pub use self::parallax::*;
}
pub use background::*;
