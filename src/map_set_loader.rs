use std::path::PathBuf;

use firecore_world::map::set::WorldMapSet;
use ahash::AHashMap as HashMap;
use crate::map_serializable::SerializedMapSet;

pub fn load_map_set(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, serialized_map_set: SerializedMapSet) -> crate::ResultT<(String, WorldMapSet)> {

    println!("Loading map set {}", serialized_map_set.identifier);

    let mut maps = Vec::new();

    for dir_string in serialized_map_set.dirs {
        let map_path = root_path.join(dir_string);
        for dir_entry in std::fs::read_dir(&map_path)? {
            let file = dir_entry?.path();
            if let Some(ext) = file.extension() {
                if ext == std::ffi::OsString::from("ron") {
                    let config = ron::from_str(
                        &std::fs::read_to_string(file)?
                    )?;
                    maps.push(
                        super::map::load_map_from_config(&map_path, palette_sizes, config)?
                    );
                }
            }
        }

        
        
    }

    Ok(
        (
            serialized_map_set.identifier,
            WorldMapSet {
                maps,
                ..Default::default()
            }
        )
    )

}