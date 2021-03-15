use std::path::PathBuf;

use firecore_world::script::world::WorldScript;

pub fn load_script_entries(root_path: &PathBuf, map_index: Option<usize>) -> Vec<WorldScript> {
    println!("Loading scripts...");
    let mut scripts = Vec::new();
    let mut script_dir = root_path.join("scripts");
    if let Some(index) = map_index {
        script_dir = script_dir.join(format!("map_{}", index));
    }
    if let Ok(dir) = std::fs::read_dir(&script_dir) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                match std::fs::read_to_string(&file) {
                    Ok(content) => {
                        let script: Result<WorldScript, ron::Error> = ron::from_str(&content);
                        match script {
                            Ok(script) => {
                                println!("Loaded script at path {:?}", file);
                                scripts.push(script)
                            },
                            Err(err) => {
                                panic!("Could not parse script at {:?} with error {} at position {}", file, err, err.position);
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Could not get script entry at {:?} as string with error {}", file, err);
                    }
                }
            }
        }
    }
    println!("Done loading scripts");
    scripts
}