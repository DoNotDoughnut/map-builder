use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    world_builder::with_dirs("world/maps", "world/textures/tiles", "world/textures/npcs", "output/world.bin")?;
    println!("Completed in {}ms!", start.elapsed().as_millis());
    Ok(())
}