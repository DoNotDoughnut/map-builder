use std::path::PathBuf;

use ahash::AHashMap as HashMap;
use crate::map_serializable::SerializedChunkMap;

use firecore_world::map::chunk::WorldChunk;

pub fn new_chunk_map(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, serialized_chunk: SerializedChunkMap) -> crate::ResultT<(u16, WorldChunk)> {
    println!("Loading chunk map {}", serialized_chunk.config.name);

    let map = super::map::load_map_from_config(root_path, palette_sizes, serialized_chunk.config)?;
    Ok(
        (
            serialized_chunk.piece_index,
            WorldChunk {
                index: serialized_chunk.piece_index,
                map,
                coords: serialized_chunk.coords,
                connections: serialized_chunk.connections,
            }
        )
    )


    // if let Some(map_file) = config.identifier.map_files.get(0) {
    //     let map_path = root_path.join(map_file);
    //     match map_path.extension() {
    //         Some(ext) => {
    //             if ext.to_string_lossy().eq("map") {
    //                 let map_file = std::fs::read(&map_path)?;
    //                 let mut gba_map = get_gba_map(map_file);
    //                 fix_tiles(&mut gba_map, palette_sizes);

    //                 let map = super::map::new_world_from_v1(
    //                     gba_map, 
    //                     &config, 
    //                     root_path, 
    //                     None
    //                 )?;
    //                 let jigsaw_map = config.jigsaw_map.unwrap();
    //                 Ok((
    //                     jigsaw_map.piece_index,
    //                     WorldChunk {
    //                         index: jigsaw_map.piece_index,
    //                         map,
    //                         coords: ,
    //                         connections: jigsaw_map.connections,
    //                     }
    //                 ))
    //             } else {
    //                 eprintln!("Could not find map {} at path {:?}", &map_file, &root_path);
    //                 Err(Box::new(super::map::MapError::UnsupportedExtension))
    //             }
    //         }
    //         None => {
    //             eprintln!("Map file at {:?} has unsupported extension!", &map_path);
    //             Err(Box::new(super::map::MapError::UnsupportedExtension))
    //         }
    //     }
    // } else {
    //     eprintln!("Map configuration did not specify any map files!");
    //     Err(Box::new(super::map::MapError::NoMaps))
    // }
    
}
