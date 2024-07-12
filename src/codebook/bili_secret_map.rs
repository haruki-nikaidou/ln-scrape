use std::collections::HashMap;
use crate::codebook::SecretMap;

const FONT_MAP_STR: &str = include_str!("bili_font.json");
const BLANK_MAP_STR: &str = include_str!("bili_blank.json");

fn parse_blank_json(json_str: &str) -> SecretMap {
    let blank_chars: Vec<char> = serde_json::from_str(json_str).expect("Failed to parse bili_blank.json");
    let mut map = HashMap::new();
    for c in blank_chars {
        map.insert(c, ' ');
    }
    SecretMap(map)
}

lazy_static::lazy_static! {
    pub static ref BILI_MAP: SecretMap = SecretMap::combine(
        SecretMap::try_from_json(FONT_MAP_STR).expect("Failed to parse bili_font.json"),
        parse_blank_json(BLANK_MAP_STR)
    );
}