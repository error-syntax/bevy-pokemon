use bevy::prelude::*;

use crate::animation::{animate_player, new_animation_timer};
use crate::camera::camera_follow;
use crate::collision::handle_collision;
use crate::components::{MoveDirection, Player, WalkAnimation};
use crate::constants::{PLAYER_SIZE, SHEET_COLS, SHEET_ROWS, SPRITE_H, SPRITE_W, VELOCITY};

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_W, SPRITE_H),
        SHEET_COLS,
        SHEET_ROWS,
        None,
        None,
    ));

    let anim = WalkAnimation { direction: MoveDirection::Down, current: 0 };
    let start_index = anim.current_frame();

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            image: asset_server.load("player_walking_sprite_sheet.png"),
            texture_atlas: Some(TextureAtlas { layout, index: start_index }),
            ..default()
        },
        anim,
        new_animation_timer(),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn handle_player_movement(
    player: Single<
        (&mut Transform, &mut WalkAnimation),
        (With<Player>, Without<Camera2d>, Without<crate::components::Wall>),
    >,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut anim) = player.into_inner();
    let mut direction = Vec2::ZERO;

    if buttons.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if buttons.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if buttons.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if buttons.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    if direction != Vec2::ZERO {
        let move_dir = if direction.y > 0.0 {
            MoveDirection::Up
        } else if direction.y < 0.0 {
            MoveDirection::Down
        } else if direction.x < 0.0 {
            MoveDirection::Left
        } else {
            MoveDirection::Right
        };
        anim.set_direction(move_dir);

        direction = direction.normalize();
        transform.translation.x += direction.x * VELOCITY;
        transform.translation.y += direction.y * VELOCITY;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (handle_player_movement, animate_player, camera_follow, handle_collision).chain(),
            );
    }
}
