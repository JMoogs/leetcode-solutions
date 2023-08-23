use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        if s.is_empty() {
            return true;
        }

        let mut keymap = HashMap::new();
        let mut rev = HashMap::new();

        let mut s = s.chars();
        let mut t = t.chars();

        while let Some(s_char) = s.next() {
            let t_char = t.next().unwrap();

            let res = keymap.insert(s_char, t_char);
            let unique = rev.insert(t_char, s_char);

            match unique {
                Some(v) if v != s_char => return false,
                _ => (),
            }

            if let Some(v) = res {
                if v != t_char {
                    return false;
                }
            }
        }

        true
    }
}
