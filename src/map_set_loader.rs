use std::path::PathBuf;

use firecore_world::map::WorldMap;
use firecore_world::map::set::WorldMapSet;
use ahash::AHashMap as HashMap;
use super::gba_map::fix_tiles;
use super::gba_map::get_gba_map;

pub fn new_map_set(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, config: super::map_serializable::MapConfig) -> crate::ResultT<(String, WorldMapSet)> {
    
    println!("Loading map set {}", &config.identifier.name);

    if config.identifier.map_files.len() == 0 {
        return Err(Box::new(super::map::MapError::NoMaps))
    }

    let mut maps: Vec<WorldMap> = Vec::with_capacity(config.identifier.map_files.len());

    for index in 0..config.identifier.map_files.len() {

        let file = std::fs::read(root_path.join(&config.identifier.map_files[index]))?;
        let mut gba_map = get_gba_map(file);
        fix_tiles(&mut gba_map, palette_sizes);

        maps.insert(
            index,
            super::map::new_world_from_v1(
                gba_map, 
                &config, 
                root_path, 
                Some(index)
            )?
        );
    }

    let wm = config.warp_map.unwrap();

    return Ok((
        wm.map_set_id.clone(),
        WorldMapSet::new(wm.map_set_id, maps),
    ));
}
