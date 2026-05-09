use bevy::{camera::ScalingMode, prelude::*};

use crate::components::{Player, Wall};

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_height: 270.,
                max_width: 480.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub(crate) fn camera_follow(
    player: Single<&Transform, (With<Player>, Without<Camera2d>, Without<Wall>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>, Without<Wall>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
