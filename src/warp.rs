use std::path::PathBuf;

use firecore_world::warp::WarpEntry;

pub fn load_warp_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<WarpEntry> {
    let mut warps = Vec::new();
    let mut warp_path = root_path.join("warps");
    if let Some(map_index) = map_index {
        warp_path = warp_path.join(String::from("map_") + &map_index.to_string());
    }
    if let Ok(dir) = std::fs::read_dir(warp_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                match std::fs::read_to_string(&file) {
                    Ok(data) => {
                        match toml::from_str(&data) {
                            Ok(warp_entry) => {
                                warps.push(warp_entry);
                            }
                            Err(err) => {
                                eprintln!("Could not parse warp entry at {:?} with error {}", &file, err);
                            }
                        }
    
                    },
                    Err(err) => {
                        eprintln!("Could not read warp entry toml at {:?} to string with error {}", &file, err);
                    }
                }
            } 
        }
    }
    return warps;
}