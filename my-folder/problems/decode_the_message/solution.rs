use std::collections::HashMap;
impl Solution {
    pub fn decode_message(mut key: String, message: String) -> String {
        // Remove whitespace
        key.retain(|c| !c.is_whitespace());

        let key = key.chars().collect::<Vec<char>>();
        let alphabet = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();

        let mut hmap = HashMap::with_capacity(27);
        
        // for spaces
        hmap.insert(' ', ' ');
        let mut alph_counter = 0;

        for i in 0..key.len() {
            if !hmap.contains_key(&key[i]) {
                hmap.insert(key[i], alphabet[alph_counter]);
                alph_counter += 1;
            }
        }


        let mut ans = String::with_capacity(message.len());

        let mut m = message.chars().map(|c| hmap.get(&c).unwrap());
        
        while let Some(next) = m.next() {
            ans.push(*next);
        }
        
        ans

    }
}