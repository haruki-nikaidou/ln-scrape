mod bili_secret_map;

use std::collections::HashMap;

pub struct SecretMap (HashMap<char,char>);

impl SecretMap {
    fn try_get(&self, key: char) -> Option<char> {
        self.0.get(&key).copied()
    }

    fn try_from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let map: HashMap<char,char> = serde_json::from_str(json_str)?;
        Ok(Self(map))
    }

    fn combine(a: Self, b: Self) -> Self {
        let mut map = a.0;
        map.extend(b.0);
        Self(map)
    }
}