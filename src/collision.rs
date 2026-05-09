use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::components::{Collidable, Player};
use crate::constants::PLAYER_SIZE;

pub(crate) fn handle_collision(
    mut player: Single<(&Sprite, &mut Transform), (With<Player>, Without<Collidable>)>,
    walls: Query<(&Sprite, &Transform), (With<Collidable>, Without<Player>)>,
) {
    let player_pos = player.1.translation.xy();
    let player_radius = PLAYER_SIZE / 2.;
    let player_collider = BoundingCircle::new(player_pos, player_radius);

    for (sprite, collidable_transform) in &walls {
        let collidable_pos = collidable_transform.translation.xy();
        let collidable_half = sprite.custom_size.unwrap() / 2.;
        let collidable_ent = Aabb2d::new(collidable_pos, collidable_half);

        if player_collider.intersects(&collidable_ent) {
            let closest = Vec2::new(
                player_pos.x.clamp(collidable_pos.x - collidable_half.x, collidable_pos.x + collidable_half.x),
                player_pos.y.clamp(collidable_pos.y - collidable_half.y, collidable_pos.y + collidable_half.y),
            );
            let offset = player_pos - closest;
            let dist = offset.length();
            let correction = if dist > 0. {
                offset / dist * (player_radius - dist)
            } else {
                Vec2::new(player_radius, 0.)
            };
            player.1.translation.x += correction.x;
            player.1.translation.y += correction.y;
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, _app: &mut App) {}
}
