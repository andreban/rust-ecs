use std::{io::Result, path::Path};

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub sprite_id: u32,
}

pub fn load_map<P: AsRef<Path>>(map_file: P) -> Result<Vec<Tile>> {
    let contents = std::fs::read_to_string(map_file)?;
    let mut tiles = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, sprite_id) in line.split(',').enumerate() {
            let sprite_id = sprite_id.parse::<u32>().unwrap();
            tiles.push(Tile { x: x as u32, y: y as u32, sprite_id });
        }
    }
    Ok(tiles)
}
