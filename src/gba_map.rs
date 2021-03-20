use std::path::PathBuf;

use ahash::AHashMap as HashMap;
use image::GenericImageView;
// use crate::image::Image;

pub struct GbaMap {
	
	pub bank: usize,
	pub map: usize,
	//pub name: String,
	pub music: u8,
	pub width: u16,
	pub height: u16,
	pub palettes: [u8; 2],
	pub border_blocks: [u16; 4],
	pub tile_map: Vec<u16>,
	pub movement_map: Vec<u8>,
	
}

pub fn get_gba_map(file: Vec<u8>) -> GbaMap  {

	let bytes = file;

	let music = bytes[40];

	let width = bytes[0] as u16; // 0 - 3 reserved
	let height = bytes[4] as u16; // 4 - 7 reserved
	
	let palettes = [bytes[8], bytes[12]]; // 8 - 11 reserved & 12 - 15 reserved
	
	//let show_name_on_entering = bytes[49];
	
	let mut border_blocks: [u16; 4] = [0; 4];
	
	for x in 0..4 {
		
		let location = 52+x*2;
		
		let tile_num = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
		
		border_blocks[x] = tile_num;
		
	}
	
	let size = width as usize * height as usize;
	
	let mut tile_map: Vec<u16> = Vec::with_capacity(size);
	let mut movement_map: Vec<u8> = Vec::with_capacity(size);
	
	for x in 0..size {
		
		let location = 60 + x * 2;
		
		let tile_num = (bytes[location+1]%4) as u16 * 256 + bytes[location] as u16;
		
		let move_num = (bytes[location+1]/4) as u8;
		
		tile_map.push(tile_num);
		movement_map.push(move_num);
		
	}

	GbaMap {
		
		bank: 0,//bank,
		map: 0, //map,
		//name: String::from(name),
		music: music,
		width: width,
		height: height,
		palettes: palettes,
		border_blocks: border_blocks,
		tile_map: tile_map,
		movement_map: movement_map,
//		spawnpoint: _spawnpoint,
		
	}
	
}

pub fn fix_tiles(gba_map: &mut GbaMap, palette_sizes: &HashMap<u8, u16>) {

	let offset = get_offset(gba_map, palette_sizes);

	let zero_size = *palette_sizes.get(&0).unwrap();
	
	for index in 0..gba_map.tile_map.len() {
		if gba_map.tile_map[index] > zero_size {
			gba_map.tile_map[index] += offset;
		}
	}

	for index in 0..gba_map.border_blocks.len() {
		if gba_map.border_blocks[index] > zero_size {
			gba_map.border_blocks[index] += offset;
		}
	}

	if gba_map.palettes[0] > 0 {
		let mut offset12: u16 = 0;

		for x in 0..gba_map.palettes[0] {
			offset12 += *palette_sizes.get(&x).unwrap();
		}

		for index in 0..gba_map.tile_map.len() {
			if gba_map.tile_map[index] < zero_size {
				gba_map.tile_map[index] += offset12;
			}
		}

		for index in 0..gba_map.border_blocks.len() {
			if gba_map.border_blocks[index] < zero_size {
				gba_map.border_blocks[index] += offset12;
			}
		}
	}

}

pub fn get_offset(gba_map: &GbaMap, palette_sizes: &HashMap<u8, u16>) -> u16 { // To - do: change to recursive function
	let mut offset = 0;
	if gba_map.palettes[1] >= palette_sizes.len() as u8 {
		eprintln!("Not enough palettes to support gba map textures. Need palette #{}", gba_map.palettes[1]);
		return 0;
	}
	for x in 1..gba_map.palettes[1] {
		offset += palette_sizes.get(&x).unwrap();
	}
	return offset;
}

pub fn fill_palette_map(tile_texture_dir: &str) -> (HashMap<u8, u16>, HashMap<u8, Vec<u8>>) {
	let mut sizes = HashMap::new();
	let mut palettes = HashMap::new();

	if let Ok(dir) = std::fs::read_dir(tile_texture_dir) {
		let paths: Vec<PathBuf> = dir.filter(|entry| entry.is_ok()).map(|entry| entry.unwrap().path()).filter(|path| path.is_file()).collect();
		for filepath in paths {
			let filename = filepath.file_name().unwrap().to_string_lossy();
			if filename.starts_with("P") {
				if filename.ends_with("B.png") {
					match filename[7..filename.len()-5].parse::<u8>() {
						Ok(index) => {
							match std::fs::read(&filepath) {
							    Ok(bytes) => {
									let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).unwrap();
									sizes.insert(index, ((img.width() >> 4) * (img.height() >> 4)) as u16);
									palettes.insert(index, bytes);
								}
							    Err(err) => {
									panic!("Could not read image at path {:?} with error {}", filepath, err);
								}
							}
						}
						Err(err) => {
							panic!("Could not parse tile palette named {} with error {}", filename, err);
						}
					}
				}
			}
		}
	}

	(sizes, palettes)

}