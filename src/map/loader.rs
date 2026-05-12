use bevy::{asset::{Asset, AssetLoader, AssetServer}, ecs::system::{Commands, Res}, reflect::TypePath};
use bevy_ecs_tiled::prelude::tiled;

// Asset definition + loader live here
#[derive(Asset, TypePath)]
pub struct TiledMap {
    pub map: tiled::Map,
}

#[derive(Default)]
pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
    type Asset;

    type Settings;

    type Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        todo!()
    }
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(CurrentMap {
        handle: asset_server.load("maps/PalletTown.tmx"),
    });
}
