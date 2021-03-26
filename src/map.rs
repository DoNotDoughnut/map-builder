use std::path::Path;
use std::path::PathBuf;
use crate::ResultT;
use crate::map_serializable::MapConfig;

use ahash::AHashMap as HashMap;

use firecore_world::map::Border;
use firecore_world::map::WorldMap;
use firecore_world::map::chunk::map::WorldChunkMap;
use firecore_world::map::manager::WorldMapManager;
use firecore_world::map::set::manager::WorldMapSetManager;
use firecore_world::serialized::Palette;
use super::gba_map::{get_gba_map, fix_tiles, fill_palette_map};

pub fn load_maps(map_dir: &str, tile_texture_dir: &str) -> ResultT<(WorldMapManager, Vec<Palette>)> {
    let mut chunk_map = WorldChunkMap::new();
    let mut map_set_manager = WorldMapSetManager::default();
    let (palette_sizes, palettes) = fill_palette_map(tile_texture_dir);
    println!("Loaded {} palettes", palette_sizes.len());

    println!("Loading maps...");

    for world_dir in std::fs::read_dir(map_dir)? {
        let world_dir = world_dir?.path();
        if let Ok(dir) = std::fs::read_dir(&world_dir) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let file = entry.path();
                    if let Some(ext) = file.extension() {
                        if ext == std::ffi::OsString::from("ron") {
                            let (cm, ms) = load_map(&palette_sizes, &world_dir, &file)?;
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

    Ok(
        (
            manager,
            palettes
        )
    )

}

fn load_map(palette_sizes: &HashMap<u8, u16>, root_path: &PathBuf, file: &PathBuf) -> ResultT<(Option<(u16, firecore_world::map::chunk::WorldChunk)>, Option<(String, firecore_world::map::set::WorldMapSet)>)> {
    
    println!("Loading map under: {:?}", root_path);
    
    let data = std::fs::read_to_string(file)?;
    let map_config: super::map_serializable::SerializedMap = ron::from_str(&data)?;
    Ok(
        if let Some(serialized_chunk) = map_config.chunk {
            (
                Some(
                    super::chunk_map_loader::new_chunk_map(root_path, palette_sizes, serialized_chunk)?
                ), 
                None
            )
        } else if let Some(serialized_map_set) = map_config.map_set {
            (
                None, 
                Some(
                    super::map_set_loader::load_map_set(root_path, palette_sizes, serialized_map_set)?
                )
            )
    } else {
        panic!("Map config at {:?} does not contain either a jigsaw map or a warp map.", &root_path);
    })
}

pub fn load_map_from_config<P: AsRef<Path>>(root_path: P, palette_sizes: &HashMap<u8, u16>, map_config: MapConfig) -> ResultT<WorldMap> {
    let root_path = root_path.as_ref();
    println!("Loading map: \"{}\"", map_config.name);
    let mut gba_map = get_gba_map(
        std::fs::read(root_path.join(&map_config.file))?
    );
    fix_tiles(&mut gba_map, palette_sizes);

    Ok(
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
            warps: crate::warp::load_warp_entries(root_path.join("warps"))?,
            wild: crate::wild::load_wild_entry(map_config.wild, root_path.join("wild")),
            npcs: crate::npc::load_npc_entries(root_path.join("npcs"))?,
            scripts: crate::script::load_script_entries(root_path.join("scripts")),
            npc_active: None,
        }
    )
}