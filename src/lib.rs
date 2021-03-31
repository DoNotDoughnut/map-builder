use std::io::Write;
use std::path::Path;

use firecore_world::map::manager::WorldMapManager;
use firecore_world::map::warp::WarpEntry;

mod world;
mod gba_map;

// pub type ResultT<T> = Result<T, Box<dyn std::error::Error>>;

pub fn compile<P: AsRef<Path>>(maps: P, tile_textures: P, npc_types: P, output_file: P) {

    println!("Started loading maps and tile textures...");
    let (manager, palettes) = world::map::load_maps(maps, tile_textures);
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
    let npc_types = world::npc::npc_type::load_npc_types(npc_types);

    let output_file = output_file.as_ref();

    if let Some(parent) = output_file.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| panic!("Could not create directories for output file with error {}", err));
        }
    }
    
    let mut file = std::fs::File::create(output_file).unwrap_or_else(|err| panic!("Could not create output file at {:?} with error {}", output_file, err));

    let data = firecore_world::serialized::SerializedWorld {
        manager,
        npc_types,
        palettes,
    };

    println!("Saving data...");
    let bytes = bincode::serialize(&data).unwrap_or_else(|err| panic!("Could not serialize output file with error {}", err));
    let bytes = file.write(&bytes).unwrap_or_else(|err| panic!("Could not write to output file with error {}", err));
    println!("Wrote {} bytes to world file!", bytes);

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
