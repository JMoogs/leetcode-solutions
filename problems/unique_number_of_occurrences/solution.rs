use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occ_counter = HashMap::new();
        for val in arr {
            match occ_counter.get(&val) {
                None => occ_counter.insert(val, 1),
                Some(v) => occ_counter.insert(val, v + 1),
            };
        }
        
        let mut set = HashSet::new();
        let mut iter = occ_counter.values(); 
        while let Some(next) = iter.next() {
            if !set.insert(next) {return false}
        }

        return true
    }
}
