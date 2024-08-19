use std::collections::HashMap;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut c_len = 0;

        let mut chars_map = HashMap::new();

        let mut c = chars.chars();

        while let Some(cha) = c.next() {

            match chars_map.get(&cha) {
                None => chars_map.insert(cha, 1),
                Some(v) => chars_map.insert(cha, v + 1),
            };
        }


        'outer: for word in words {
            let mut characters = word.chars();

            let mut word_map = HashMap::new();

            while let Some(ch) = characters.next() {

                match word_map.get(&ch) {
                    None => word_map.insert(ch, 1),
                    Some(v) => word_map.insert(ch, v + 1),
                };

                match chars_map.get(&ch) {
                    None => continue 'outer,
                    Some(value) => {
                        if value < word_map.get(&ch).unwrap() {
                            continue 'outer;
                        }

                    }
                };

            }

            c_len += word.len()
        }

        c_len as i32
    }

}
