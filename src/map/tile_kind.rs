use bevy::ecs::component::Component;
use bevy_ecs_tiled::prelude::tiled;

#[derive(Component, Debug)]
#[allow(dead_code)]
pub enum TileKind {
    Passable,
    Impassable,
    WildGrass,
    Water,
    Door { target_map: String, target_x: i32, target_y: i32 },
    Warp { target_map: String, target_x: i32, target_y: i32 },
}

impl TileKind {
    pub fn from_tile(tile: &tiled::Tile) -> Self {
        let props = &tile.properties;

        match props.get("kind") {
            Some(tiled::PropertyValue::StringValue(s)) => match s.as_str() {
                "Impassable" => TileKind::Impassable,
                "WildGrass"  => TileKind::WildGrass,
                "Water"      => TileKind::Water,
                "Door"       => TileKind::Door {
                    target_map: str_prop(props, "target_map").unwrap_or("").to_owned(),
                    target_x: int_prop(props, "target_x").unwrap_or(0),
                    target_y: int_prop(props, "target_y").unwrap_or(0),
                },
                "Warp"       => TileKind::Warp {
                    target_map: str_prop(props, "target_map").unwrap_or("").to_owned(),
                    target_x: int_prop(props, "target_x").unwrap_or(0),
                    target_y: int_prop(props, "target_y").unwrap_or(0),
                },
                _ => TileKind::Passable,
            },
            _ => TileKind::Passable,
        }
    }
}

fn str_prop<'a>(props: &'a tiled::Properties, key: &str) -> Option<&'a str> {
    match props.get(key) {
        Some(tiled::PropertyValue::StringValue(s)) => Some(s),
        _ => None,
    }
}

fn int_prop(props: &tiled::Properties, key: &str) -> Option<i32> {
    match props.get(key) {
        Some(tiled::PropertyValue::IntValue(n)) => Some(*n),
        _ => None,
    }
}
