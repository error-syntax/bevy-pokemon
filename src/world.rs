use bevy::prelude::*;

use crate::components::{Collidable, Wall};

fn spawn_walls(mut commands: Commands) {
    commands.spawn((
        Wall,
        Collidable,
        Sprite {
            custom_size: Some(Vec2::splat(30.)),
            color: Color::linear_rgb(255., 255., 255.),
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 1.0),
    ));
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}
