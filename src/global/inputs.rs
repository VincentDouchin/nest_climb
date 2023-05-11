use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerInputs {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}
impl Default for PlayerInputs {
    fn default() -> Self {
        PlayerInputs {
            left: false,
            right: false,
            jump: false,
        }
    }
}
impl PlayerInputs {
    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.jump = false
    }
}

fn update_inputs(mut player_inputs: ResMut<PlayerInputs>, keys: Res<Input<KeyCode>>) {
    player_inputs.reset();
    if keys.pressed(KeyCode::Left) {
        player_inputs.left = true
    }
    if keys.pressed(KeyCode::Right) {
        player_inputs.right = true
    }
}
pub fn inputs_plugin(app: &mut App) {
    app.insert_resource(PlayerInputs::default());
    app.add_system(update_inputs);
}
