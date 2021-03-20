use std::io::Write;
use std::path::Path;

use firecore_world::map::manager::WorldMapManager;
use firecore_world::map::warp::WarpEntry;

mod chunk_map_loader;
mod map_set_loader;
mod map;
mod npc;
mod script;
mod warp;
mod wild;
mod npc_type;

pub mod map_serializable;
pub mod gba_map;
// pub mod image;


pub type ResultT<T> = Result<T, Box<dyn std::error::Error>>;

pub fn compile(map_dir: &str, tile_texture_dir: &str, npc_type_dir: &str, output_file: &str) -> ResultT<()> {

    println!("Started loading maps and tile textures...");
    let (manager, palettes) = map::load_maps(map_dir, tile_texture_dir)?;
    println!("Finished loading maps and tile textures.");

    println!("Verifying maps and warps...");
    for chunk in manager.chunk_map.chunks.values() {
        for connection in chunk.connections.iter() {
            if !manager.chunk_map.chunks.contains_key(connection) {
                panic!("Map {} contains a connection to non-existent index {}", chunk.map.name, connection);
            }
        }
        for warp in chunk.map.warps.iter() {
            verify_warp(warp, &chunk.map.name, &manager);
        }
    }
    for map_set in manager.map_set_manager.map_sets.values() {
        for map in map_set.maps.iter() {
            for warp in map.warps.iter() {
                verify_warp(warp, &map.name, &manager);
            }
        }
    }

    println!("Loading NPC types...");
    let npc_types = npc_type::load_npc_types(npc_type_dir)?;

    if let Some(parent) = Path::new(output_file).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    let mut file = std::fs::File::create(output_file)?;

    let data = firecore_world::serialized::SerializedWorld {
        manager,
        npc_types,
        palettes,
    };

    println!("Saving data...");
    let bytes = bincode::serialize(&data)?;
    let bytes = file.write(&bytes)?;
    println!("Wrote {} bytes to world file!", bytes);
    
    Ok(())
}

fn verify_warp(warp: &WarpEntry, map_name: &String, manager: &WorldMapManager) {
    if warp.destination.map_id.as_str().eq("world") {
        if !manager.chunk_map.chunks.contains_key(&warp.destination.map_index) {
            panic!("Map {} contains a warp to non-existent chunk index {}", map_name, warp.destination.map_index);
        }
    } else if let Some(map_set) = manager.map_set_manager.map_sets.get(&warp.destination.map_id) {
        if map_set.maps.len() <= warp.destination.map_index as usize {
            panic!("Map {} contains a warp to a non-existent map at index {} in map set {}", map_name, warp.destination.map_index, map_set.name);
        }
    } else {
        panic!("Map {} contains a warp to non-existent map set {}", map_name, warp.destination.map_id);
    }
}
