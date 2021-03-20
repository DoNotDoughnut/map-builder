use std::path::PathBuf;
use ahash::AHashMap as HashMap;

use firecore_world::character::npc::NPC;

pub fn load_npc_entries(npc_path: PathBuf) -> crate::ResultT<HashMap<u8, NPC>> {
    let mut npcs = HashMap::new();
    if let Ok(dir) = std::fs::read_dir(npc_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                let data = std::fs::read_to_string(&file)?;
                let npc_result: Result<NPC, ron::Error> = ron::from_str(&data);
                match npc_result {
                    Ok(npc) => {
                        npcs.insert(npc.identifier.index, npc);
                    },
                    Err(err) => {
                        panic!("Could not parse NPC at {:?} with error {} at position {}", file, err, err.position);
                    },
                }
            }
        }
    } 
    Ok(npcs)
}