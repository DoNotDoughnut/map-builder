use std::path::Path;
use std::path::PathBuf;

use ahash::AHashMap as HashMap;

use firecore_world::map::*;
use firecore_world::map::chunk::map::WorldChunkMap;
use firecore_world::map::manager::WorldMapManager;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::serialized::Palette;
use crate::gba_map::{get_gba_map, fix_tiles, fill_palette_map};

use super::MapConfig;

pub mod chunk;
pub mod set;

pub fn load_maps<P: AsRef<Path>>(maps: P, tile_textures: P) -> (WorldMapManager, Vec<Palette>) {

    let maps = maps.as_ref();
    let tile_textures = tile_textures.as_ref();

    let mut chunk_map = WorldChunkMap::new();
    let mut map_set_manager = WorldMapSetManager::default();
    let (palette_sizes, palettes) = fill_palette_map(tile_textures);
    println!("Loaded {} palettes", palette_sizes.len());

    println!("Loading maps...");

    for worlds in std::fs::read_dir(maps).unwrap_or_else(|err| panic!("Could not read directory at {:?} with error {}", maps, err)) {
        let worlds = worlds.unwrap_or_else(|err| panic!("Could not get directory entry under {:?} with error {}", maps, err)).path();
        if let Ok(dir) = std::fs::read_dir(&worlds) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if let Some(ext) = file.extension() {
                        if ext == std::ffi::OsString::from("ron") {
                            let (cm, ms) = load_map(&palette_sizes, &worlds, &file);
                            if let Some((index, chunk)) = cm {
                                chunk_map.chunks.insert(index, chunk);
                            } else if let Some((index, map_set)) = ms {
                                map_set_manager.map_sets.insert(index, map_set);
                            }
                        }
                    }
                }
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

    let manager = WorldMapManager {
        chunk_map,
        map_set_manager,
        ..Default::default()
    };

    (
        manager,
        palettes
    )

}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> (Option<(u16, firecore_world::map::chunk::WorldChunk)>, Option<(String, firecore_world::map::set::WorldMapSet)>) {
    
    println!("Loading map under: {:?}", root_path);
    
    let data = std::fs::read_to_string(file).unwrap_or_else(|err| panic!("Could not read map configuration file at {:?} to string with error {}", file, err));
    let map_config: super::SerializedMap = ron::from_str(&data).unwrap_or_else(|err| panic!("Could not deserialize map configuration at {:?} with error {}", file, err));
    if let Some(serialized_chunk) = map_config.chunk {
        (
            Some(
                chunk::new_chunk_map(root_path, palette_sizes, serialized_chunk)
            ), 
            None
        )
    } else if let Some(serialized_map_set) = map_config.map_set {
        (
            None, 
            Some(
                set::load_map_set(root_path, palette_sizes, serialized_map_set)
            )
        )
    } else {
        panic!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &root_path);
    }
}

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, palette_sizes: &HashMap<u8, u16>, map_config: MapConfig) -> WorldMap {
    let root_path = root_path.as_ref();
    // println!("Loading map: \"{}\"", map_config.name);
    let mut gba_map = get_gba_map(
        std::fs::read(root_path.join(&map_config.file)).unwrap_or_else(|err| panic!("Could not get map file at {:?} with error {}", root_path, err))
    );
    fix_tiles(&mut gba_map, palette_sizes);

    WorldMap {
        name: map_config.name,
        music: gba_map.music,
        width: gba_map.width,
        height: gba_map.height,
        tiles: gba_map.tiles,
        border: Border {
            tiles: gba_map.borders.into(),
            size: (gba_map.borders.len() as f32).sqrt() as u8,
        },
        movements: gba_map.movements,
        warps: super::warp::load_warp_entries(root_path.join("warps")),
        wild: super::wild::load_wild_entry(map_config.wild, root_path.join("wild")),
        npcs: super::npc::load_npc_entries(root_path.join("npcs")),
        scripts: super::script::load_script_entries(root_path.join("scripts")),
        npc_active: None,
        npc_timer: default_npc_timer(),
    }
}