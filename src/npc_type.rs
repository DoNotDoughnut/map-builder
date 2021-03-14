use firecore_world::serialized::NPCType;

pub fn load_npc_types(npc_type_dir: &str) -> Vec<NPCType> {
    let mut types = Vec::new();

    if let Ok(dir) = std::fs::read_dir(npc_type_dir) {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() {
                    let sprite_path = path.join(name.clone() + ".png");
                    let battle_sprite_path = path.join("battle.png");
                    match std::fs::read(sprite_path) {
                        Ok(bytes) => {
                            println!("Added NPC type {}!", &name);
                            let mut npc_type = NPCType {
                                name,
                                sprite: bytes,
                                battle_sprite: None,
                            };
                            if let Ok(bytes) = std::fs::read(battle_sprite_path) {
                                npc_type.battle_sprite = Some(bytes);
                            }
                            types.push(npc_type);
                        }
                        Err(err) => {
                            eprintln!("Could not read npc type {}'s sprite with error {}", name, err);
                        }
                    }
                }
            }
        }
    }

    types
}