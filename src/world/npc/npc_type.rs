use std::path::Path;

use firecore_world::serialized::SerializedNPCType;

pub fn load_npc_types<P: AsRef<Path>>(npc_types: P) -> Vec<SerializedNPCType> {
    let npc_types = npc_types.as_ref();
    let mut types = Vec::new();

    for entry in std::fs::read_dir(npc_types)
    .unwrap_or_else(|err| panic!("Could not get warp file at {:?} with error {}", npc_types, err))
        .map(|entry| entry.unwrap_or_else(|err| panic!("Could not directory entry at {:?} with error {}", npc_types, err))) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            let data_path = path.join(name.clone() + ".ron");
            let sprite_path = path.join(name.clone() + ".png");
            let battle_sprite_path = path.join("battle.png");
            let bytes =  std::fs::read(&sprite_path).unwrap_or_else(|err| panic!("Could not get npc sprite at {:?} with error {}", sprite_path, err));
            let data = ron::from_str(
                &std::fs::read_to_string(&data_path).unwrap_or_else(|err| panic!("Could not get NPC type file at {:?} with error {}", data_path, err))
            ).unwrap_or_else(|err| panic!("Could not decode NPC type file at {:?} with error {}", data_path, err));
            println!("Added NPC type {}!", &name);
            let mut npc_type = SerializedNPCType {
                identifier: name,
                data,
                sprite: bytes,
                battle_sprite: None,
            };
            if let Ok(bytes) = std::fs::read(battle_sprite_path) {
                npc_type.battle_sprite = Some(bytes);
            }
            types.push(npc_type);
        }
    }

    types
}