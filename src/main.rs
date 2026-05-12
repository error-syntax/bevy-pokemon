mod animation;
mod camera;
mod collision;
mod components;
mod constants;
mod map;
mod player;
mod state;

use animation::AnimationPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use map::plugin::MapPlugin;
use player::PlayerPlugin;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_plugins((
            MapPlugin,
            PlayerPlugin,
            AnimationPlugin,
            CameraPlugin,
            CollisionPlugin,
        ))
        .run();
}
