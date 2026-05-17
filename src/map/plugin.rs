use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::map::spawner::{attach_tile_kinds, hide_collision_layer};
use crate::state::GameState;

pub struct MapPlugin;

#[derive(Resource)]
pub struct CurrentMap {
  pub handle: Handle<TiledMapAsset>
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin(TiledPluginConfig {
                tiled_types_filter: TiledFilter::All,
                tiled_types_export_file: None,
            }))
            .add_systems(Startup, spawn_map)
            .add_systems(Update, (
                attach_tile_kinds,
                hide_collision_layer,
                transition_to_playing.run_if(in_state(GameState::LoadingMap)),
            ));
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handle: Handle<TiledMapAsset> = asset_server.load("maps/PalletTown.tmx");
  commands.spawn(TiledMap(handle.clone()));
  commands.insert_resource(CurrentMap { handle });
}

fn transition_to_playing(
    mut events: MessageReader<TiledEvent<MapCreated>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if events.read().next().is_some() {
        next_state.set(GameState::Playing);
    }
}
