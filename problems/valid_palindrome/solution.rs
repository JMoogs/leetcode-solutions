impl Solution {
pub fn is_palindrome(mut s: String) -> bool {
        s.retain(|c| c.is_ascii_alphanumeric());
        s.make_ascii_uppercase();

        let mut r = s.clone().into_bytes();
        r.reverse();

        s.into_bytes() == r
    }
}