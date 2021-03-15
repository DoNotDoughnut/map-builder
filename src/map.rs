use std::path::PathBuf;
use crate::ResultT;

use super::gba_map;

use ahash::AHashMap as HashMap;

use firecore_world::map::WorldMap;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::serialized::Palette;

pub fn load_maps(map_dir: &str, tile_texture_dir: &str) -> ResultT<(WorldChunkMap, WorldMapSetManager, Vec<Palette>)> {
    let mut chunk_map = WorldChunkMap::new();
    let mut map_set_manager = WorldMapSetManager::default();
    let (palette_sizes, palettes) = gba_map::fill_palette_map(tile_texture_dir);
    println!("Loaded {} palettes", palette_sizes.len());

    println!("Loading maps...");
    for world_dir in std::fs::read_dir(map_dir)? {
        match world_dir {
            Ok(world_dir) => {
                let world_dir = world_dir.path();
                println!("Dir: {:?}", world_dir);
                if let Ok(dir) = std::fs::read_dir(&world_dir) {
                    for entry in dir {
                        if let Ok(entry) = entry {
                            let file = entry.path();
                            if let Some(ext) = file.extension() {
                                if ext == std::ffi::OsString::from("toml") {
                                    let (cm, ms) = load_map(&palette_sizes, &world_dir, &file)?;
                                    if let Some(world_chunk) = cm {
                                        chunk_map.chunks.insert(world_chunk.0, world_chunk.1);
                                    } else if let Some(map_set) = ms {
                                        map_set_manager.map_sets.insert(map_set.0, map_set.1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
        
    }

    let palettes = palettes.into_iter().map(
        |(id, bottom)|
        Palette {
            id,
            bottom,
        }
    ).collect();

    println!("Finished loading maps!");

    Ok((
        chunk_map,
        map_set_manager,
        palettes
    ))

}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> ResultT<(Option<(u16, firecore_world::map::chunk::WorldChunk)>, Option<(String, firecore_world::map::set::WorldMapSet)>)> {
    let data = std::fs::read_to_string(file)?;
    let map_config = super::map_serializable::MapConfig::from_string(&data)?;
    Ok(if map_config.jigsaw_map.is_some() {
        (Some(super::chunk_map_loader::new_chunk_map(root_path, palette_sizes, map_config)?), None)
    } else if map_config.warp_map.is_some() {
        (None, Some(super::map_set_loader::new_map_set(root_path, palette_sizes, map_config)?))
    } else {
        eprintln!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &root_path);
        (None, None)
    })
}

pub fn new_world_from_v1(gba_map: gba_map::GbaMap, config: &super::map_serializable::MapConfig, root_path: &PathBuf, map_index: Option<usize>) -> ResultT<WorldMap> {
    Ok(WorldMap {
        name: config.identifier.name.clone(),
        music: gba_map.music,
        width: gba_map.width,
        height: gba_map.height,
        tile_map: gba_map.tile_map,
        border_blocks: gba_map.border_blocks,
        movement_map: gba_map.movement_map,
        fly_position: config.settings.fly_position,
        wild: crate::wild::load_wild_entry(root_path, config.wild.clone(), map_index),
        objects: HashMap::new(),
        warps: crate::warp::load_warp_entries(root_path, map_index)?,
        npcs: crate::npc::load_npc_entries(root_path, map_index)?,
        scripts: crate::script::load_script_entries(root_path, map_index),
        script_npcs: HashMap::new(),
        npc_active: None,
    })
}

#[derive(Debug)]
pub enum MapError {

    UnsupportedExtension,
    NoMaps,

}

impl std::error::Error for MapError {}

impl core::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}