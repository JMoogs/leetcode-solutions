impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
                s.split_whitespace().next_back().unwrap().len() as i32

    }
}