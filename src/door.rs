use bevy::app::{Plugin, Update};

pub struct DoorPlugin;

fn handle_door_collision(
) {

}

impl Plugin for DoorPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_systems(Update, handle_door_collision);
  }
}
