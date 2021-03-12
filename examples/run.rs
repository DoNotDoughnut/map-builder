use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    map_compressor::with_dirs("maps", "tiles", "output")?;
    println!("Completed in {}ms!", start.elapsed().as_millis());
    Ok(())
}