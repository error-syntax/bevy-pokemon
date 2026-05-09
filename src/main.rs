mod animation;
mod camera;
mod collision;
mod components;
mod constants;
mod player;
mod world;

use animation::AnimationPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((PlayerPlugin, AnimationPlugin, CameraPlugin, WorldPlugin, CollisionPlugin))
        .run();
}
