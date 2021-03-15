use std::time::Instant;

static OUTPUT: &str = "output/world.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let start = Instant::now();
    world_builder::with_dirs("world/maps", "world/textures/tiles", "world/textures/npcs", OUTPUT)?;
    println!("Completed in {}ms!", start.elapsed().as_millis());

    match std::fs::read(OUTPUT) {
        Ok(bytes) => {
            let result: Result<firecore_world::serialized::SerializedWorld, bincode::Error> = bincode::deserialize(&bytes);
            match result {
                Ok(world) => {
                    println!("Successfully decoded serialized world!");
                    for palette in &world.palettes {
                        if palette.id == 0 {
                            match std::fs::read("world/textures/tiles/Palette0B.png") {
                                Ok(bytes) => {
                                    if palette.bottom.len() == bytes.len() {
                                        if palette.bottom == bytes {
                                            println!("Palette is equal to file!");
                                        }
                                    } else {
                                        println!("Palette 0 is not equal to file!");
                                    }
                                }
                                Err(err) => {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }

                }
                Err(err) => {
                    eprintln!("Could not decode serialized world with error {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Could not read output file with error {}", err);
        }
    }
    Ok(())
}