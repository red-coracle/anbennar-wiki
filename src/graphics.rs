use std::collections::HashMap;
use std::fs;

use jomini::TextTape;

use crate::utils::read_all_files_in_path;

#[derive(Debug, Default)]
pub struct SpriteType {
    name: String,
    texture_file: String,
}

pub fn parse_sprites() -> HashMap<String, SpriteType> {
    let sprites = HashMap::new();

    let paths = read_all_files_in_path("./anbennar/interface".to_string());
    for path in paths {
        if path.extension().unwrap_or("".as_ref()) == "gfx" {
            let data = fs::read(path).expect("error reading file");
            let tape = TextTape::from_slice(data.as_slice()).unwrap();
            let reader = tape.windows1252_reader();
            for (key, _op, value) in reader.fields() {
                if key.read_str() == "spriteTypes" {
                    if let Ok(value) = value.read_object() {
                        for (key, _op, value) in value.fields() {
                            todo!()
                        }
                    }
                }
            }
        }
    }

    sprites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sprites() {
        // parse_sprites();
    }
}