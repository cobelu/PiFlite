use leaflet::{Map, TileLayer};
use wasm_bindgen::prelude::*;
use leaflet::MapOptions;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let options = MapOptions::default();
    let map = Map::new("map", &options);

    add_tile_layer(&map);

    Ok(())
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
