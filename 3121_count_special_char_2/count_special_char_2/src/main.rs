use std::collections::HashMap;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut map: HashMap<char, u8> = HashMap::new();
        for c in word.chars() {
            if c.is_lowercase() && map.get(&c) == Some(&2) {
                map.insert(c, 0);
            }
            else if c.is_lowercase() {
                map.entry(c).or_insert(1);
            }
            else if c.is_uppercase() && map.get(&c.to_ascii_lowercase()) == Some(&1) {
                map.insert(c.to_ascii_lowercase(), 2);
            }
            else if c.is_uppercase() && map.get(&c.to_ascii_lowercase()) != Some(&2){
                map.insert(c.to_ascii_lowercase(), 0);
            }
        }

        let mut result = 0;
        for &v in map.values() {
            if v == 2 {
                result += 1;
            }
        }

        result
    }
}