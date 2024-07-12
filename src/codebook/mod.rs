use std::collections::HashMap;

pub struct SecretMap (HashMap<char,char>);

impl SecretMap {
    fn try_get(&self, key: char) -> Option<char> {
        self.0.get(&key).copied()
    }
}