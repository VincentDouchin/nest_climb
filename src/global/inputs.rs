use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Left,
    Right,
    Up,
    Down,
    Pause,
    Select,
}
pub fn get_menu_input_map() -> InputMap<MenuAction> {
    use MenuAction::*;
    let mut input_map = InputMap::default();

    // Pause
    input_map.insert(KeyCode::Escape, Pause);
    input_map.insert(GamepadButtonType::Start, Pause);

    return input_map;
}

pub fn spawn_inputs(mut commands: Commands) {
    dbg!(get_menu_input_map());
    commands.spawn(InputManagerBundle::<MenuAction> {
        input_map: get_menu_input_map(),
        ..default()
    });
}
