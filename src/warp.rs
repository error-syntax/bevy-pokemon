use bevy::{
  app::{App, Plugin, Update},
  asset::AssetServer,
  ecs::{
    query::With,
    system::{Query, Res, ResMut}
  },
  prelude::*,
  state::{condition::in_state, state::NextState}
};
use bevy_ecs_tilemap::prelude::{TilePos, TilemapGridSize, TilemapId};

use crate::{components::Player, map::{plugin::CurrentMap, tile_kind::TileKind}, state::GameState};

pub struct WarpPlugin;

fn check_warp(
    player: Query<&Transform, With<Player>>,
    tiles: Query<(&TilePos, &TilemapId, &TileKind)>,
    tilemaps: Query<(&GlobalTransform, &TilemapGridSize)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<CurrentMap>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player_transform) = player.single() else { return };
    let player_pos = player_transform.translation.xy();

    for (tile_pos, tilemap_id, tile_kind) in &tiles {
        let Ok((tilemap_transform, grid_size)) = tilemaps.get(tilemap_id.0) else { continue };

        let tile_origin = tilemap_transform.translation().xy();
        let tile_center = tile_origin + Vec2::new(
            tile_pos.x as f32 * grid_size.x,
            tile_pos.y as f32 * grid_size.y,
        );
        let tile_half = Vec2::new(grid_size.x / 2., grid_size.y / 2.);

        let within_tile = player_pos.x >= tile_center.x - tile_half.x
            && player_pos.x <= tile_center.x + tile_half.x
            && player_pos.y >= tile_center.y - tile_half.y
            && player_pos.y <= tile_center.y + tile_half.y;

        if !within_tile { continue }

        match tile_kind {
            TileKind::Door { target_map, target_x, target_y } => {
                info!(
                    "Player entered door at tile ({}, {}) -> map: {}, pos: ({}, {})",
                    tile_pos.x, tile_pos.y, target_map, target_x, target_y
                );
                // current_map.handle = asset_server.load(
                //     format!("maps/{}.tmx", target_map)
                // );
                // next_state.set(GameState::LoadingMap);
            }
            TileKind::Warp { target_map, target_x, target_y } => {
                current_map.handle = asset_server.load(
                    format!("maps/{}.tmx", target_map)
                );
                next_state.set(GameState::LoadingMap);
            }
            _ => {}
        }
    }
}

impl Plugin for WarpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
          Update,
          check_warp.run_if(in_state(GameState::Playing))
        );
    }
}
