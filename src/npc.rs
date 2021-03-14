use std::path::PathBuf;

use firecore_world::character::npc::NPC;

pub fn load_npc_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<NPC> {
    let mut npcs = Vec::new();
    let mut npc_dir = root_path.join("npcs");
    if let Some(map_index) = map_index {
        npc_dir = npc_dir.join(String::from("map_") + &map_index.to_string());
    }
    if let Ok(dir) = std::fs::read_dir(npc_dir) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                match std::fs::read_to_string(&file) {
                    Ok(data) => {
                        let npc_result: Result<NPC, ron::Error> = ron::from_str(&data);
                        match npc_result {
                            Ok(npc) => {
                                println!("Loaded NPC {}", &npc.identifier.name);
                                npcs.push(npc);
                            },
                            Err(err) => {
                                eprintln!("Could not parse NPC .ron at {:?} with error {}", file, err);
                            },
                        }
                    },
                    Err(err) => {
                        eprintln!("Could not get NPC json at {:?} with error {}", file, err);
                    },
                }
            }
        }
    } 
    return npcs;
}