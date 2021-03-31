use serde::Deserialize;

use firecore_util::Coordinate;

pub mod map;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;

#[derive(Deserialize)]
pub struct SerializedMap {

    pub chunk: Option<SerializedChunkMap>,
    pub map_set: Option<SerializedMapSet>,

}

#[derive(Deserialize)]
pub struct MapConfig {

    pub name: String,
    pub file: String,

    #[serde(default)]
    pub settings: SerializedMapSettings,
    pub wild: Option<SerializedWildEntry>,

}

#[derive(Deserialize)]
pub struct SerializedChunkMap {

    pub config: MapConfig,

    pub piece_index: u16,
    pub coords: Coordinate,
    pub connections: smallvec::SmallVec<[u16; 6]>,

}

#[derive(Deserialize)]
pub struct SerializedMapSet {

    pub identifier: String,
    pub dirs: Vec<String>,

}

#[derive(Default, Deserialize)]
pub struct SerializedMapSettings {

    pub fly_position: Option<Coordinate>,

}

#[derive(Deserialize, Clone)]
pub struct SerializedWildEntry {

    #[serde(rename = "type")]
    pub encounter_type: String,
    #[serde(default)]
    pub tiles: Option<Vec<u16>>,
    
}