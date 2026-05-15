use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::{TilePos, TilemapGridSize, TilemapId};

use crate::components::Player;
use crate::constants::PLAYER_SIZE;
use crate::map::tile_kind::TileKind;

fn handle_collision(
    player: Single<&mut Transform, With<Player>>,
    impassable: Query<(&TilePos, &TilemapId, &TileKind)>,
    tilemaps: Query<(&GlobalTransform, &TilemapGridSize)>,
) {
    let mut transform = player.into_inner();
    let player_radius = PLAYER_SIZE / 2.;

    for (tile_pos, tilemap_id, kind) in &impassable {
      if !matches!(kind, TileKind::Impassable | TileKind::Water) { continue }

        let (tile_center, tile_half) = get_tile_collider(&tilemaps, &tile_pos, tilemap_id);

        if tile_center == Vec2::default() && tile_half == Vec2::default() { continue }

        let player_pos = transform.translation.xy();
        let player_collider = BoundingCircle::new(player_pos, player_radius);
        let tile_aabb = Aabb2d::new(tile_center, tile_half);

        if player_collider.intersects(&tile_aabb) {
            let closest = Vec2::new(
                player_pos.x.clamp(tile_center.x - tile_half.x, tile_center.x + tile_half.x),
                player_pos.y.clamp(tile_center.y - tile_half.y, tile_center.y + tile_half.y),
            );
            let offset = player_pos - closest;
            let dist = offset.length();
            let correction = if dist > 0. {
                offset / dist * (player_radius - dist)
            } else {
                Vec2::new(player_radius, 0.)
            };
            transform.translation.x += correction.x;
            transform.translation.y += correction.y;
        }
    }
}

fn get_tile_collider(tilemaps: &Query<(&GlobalTransform, &TilemapGridSize)>, tile_pos: &TilePos, tilemap_id: &TilemapId) -> (Vec2, Vec2) {
  let Ok((tilemap_transform, grid_size)) = tilemaps.get(tilemap_id.0) else {  return (Vec2::default(), Vec2::default()) };
  let tile_origin = tilemap_transform.translation().xy();
  let tile_center = tile_origin + Vec2::new(
    tile_pos.x as f32 * grid_size.x,
    tile_pos.y as f32 * grid_size.y,
  );
  let tile_half = Vec2::new(grid_size.x / 2., grid_size.y / 2.);

  (tile_center, tile_half)
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collision);
    }
}
