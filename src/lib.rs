use std::io::Write;
use std::path::Path;

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
pub mod image;

pub fn with_dirs(map_dir: &str, tile_texture_dir: &str, npc_type_dir: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    println!("Loading maps...");
    let (chunks, map_sets, palettes) = map::load_maps(map_dir, tile_texture_dir);
    println!("Finished loading maps.");

    println!("Loading NPC types...");
    let npc_types = npc_type::load_npc_types(npc_type_dir);

    if let Some(parent) = Path::new(output_file).parent() {
        if !parent.exists() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("Could not create directory for output file with error {}", err);
            }
        }
    }
    
    let mut file = std::fs::File::create(output_file)?;

    let data = firecore_world::serialized::SerializedWorld {
        chunks,
        map_sets,
        npc_types,
        palettes,
    };

    println!("Saving data...");
    let bytes = bincode::serialize(&data)?;
    let bytes = file.write(&bytes)?;
    println!("Wrote {} bytes to world file!", bytes);
    
    Ok(())
}

