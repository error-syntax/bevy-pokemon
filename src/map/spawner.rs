use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::map::tile_kind::TileKind;

pub fn attach_tile_kinds(
    mut events: MessageReader<TiledEvent<TileCreated>>,
    map_assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let Some(layer) = event.get_layer(&map_assets) else { continue };
        if layer.name != "CollisionLayer" { continue }

        let Some(tile) = event.get_tile(&map_assets) else { continue };
        let Some(entity) = event.get_tile_entity() else { continue };

        commands.entity(entity).insert(TileKind::from_tile(&tile));
    }
}

pub fn hide_collision_layer(
    mut events: MessageReader<TiledEvent<LayerCreated>>,
    map_assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let Some(layer) = event.get_layer(&map_assets) else { continue };
        if layer.name == "CollisionLayer" {
            commands.entity(event.origin).insert(Visibility::Hidden);
        }
    }
}
