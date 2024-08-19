impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let len = words.len();

        let mut f_dist: Option<usize> = None;
        let mut b_dist: Option<usize> = None;

        for i in 0..len {
            if words[(i + start_index as usize) % len] == target {
                f_dist = Some(i);
                break;
            }
        }

        for i in 1..=len {
            if words[((len - i) + start_index as usize) % len] == target {
                b_dist = Some(i);
                break;
            }
        }

        match (f_dist, b_dist) {
            (Some(v), None) => v as i32,
            (None, Some(v)) => v as i32,
            (None, None) => -1,
            (Some(v), Some(v2)) => {
                if v > v2 {
                    return v2 as i32;
                } else {
                    return v as i32;
                }
            }
        }
    }
}
