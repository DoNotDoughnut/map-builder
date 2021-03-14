use std::path::PathBuf;

use firecore_world::wild::WildEntry;
use firecore_world::wild::table::WildPokemonTable;

pub fn load_wild_entry(root_path: &PathBuf, wild: Option<super::map_serializable::SerializedWildEntry>, map_index: Option<usize>) -> Option<WildEntry> {
    wild.map(|toml_wild_entry| {
        let mut wild_path = root_path.join("wild");

        if let Some(map_index) = map_index {
            wild_path = wild_path.join(String::from("map_") + &map_index.to_string());
        }

        let file = wild_path.join("grass.toml");

        let table = match toml_wild_entry.encounter_type.as_str() {
            "original" => {
                match std::fs::read_to_string(&file) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(table) => table,
                            Err(err) => {
                                eprintln!("Could not parse wild pokemon table at {:?} with error {}, using random table instead!", &file, err);
                                WildPokemonTable::default()
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not find wild toml file at {:?} with error {}!", file, err);
                        WildPokemonTable::default()
                    }
                }
            }
            _ => {
                WildPokemonTable::default()
            }
        };

        WildEntry {
            tiles: toml_wild_entry.wild_tiles,
            table: table,
        }

    })
}