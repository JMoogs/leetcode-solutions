use std::collections::HashSet;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let len = s.len();
        let mut map = HashSet::new();
        let mut acc = 0;
        for c in s.chars() {
            // We added it a second time so we have a palindrome pair
            if !map.insert(c) {
                acc += 2;
                // Reset the map
                map.remove(&c);
            }
        }

        // Case where we can add an extra character in the middle
        if len as i32 > acc {
            return acc + 1;
        }
        return acc;
    }

}
