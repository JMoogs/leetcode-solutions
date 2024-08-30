impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let max = s.len() - 1;
        if s.len() == 0 {
            return;
        }
        let mut i = 0;

        while i < max - i {
            s.swap(i, max - i);
            i += 1;
        }
    }


}