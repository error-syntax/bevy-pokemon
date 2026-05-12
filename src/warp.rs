// Warp gets its own plugin because it's a distinct gameplay system
pub struct WarpPlugin;

impl Plugin for WarpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_warp
            .run_if(in_state(GameState::Playing))
        );
    }
}

fn check_warp(
    player: Query<&TilePos, (With<Player>, Changed<TilePos>)>,
    tiles: Query<(&TilePos, &TileKind)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<CurrentMap>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player_pos) = player.get_single() else { return };

    for (tile_pos, tile_kind) in &tiles {
        if tile_pos != player_pos { continue }

        match tile_kind {
            TileKind::Door { target_map, target_x, target_y } |
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
