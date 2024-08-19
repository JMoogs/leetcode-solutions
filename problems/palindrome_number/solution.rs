impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false}
        let s = x.to_string();
        let sr: String = s.chars().rev().collect();
        if s == sr {return true}
        false
    }
}