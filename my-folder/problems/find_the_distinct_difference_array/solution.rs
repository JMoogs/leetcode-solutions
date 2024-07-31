use std::collections::HashSet;
impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut diff = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            let dd = get_diff_elements(&nums[0..i + 1]) as i32
                - get_diff_elements(&nums[i + 1..nums.len()]) as i32;
            diff.push(dd);
        }

        diff
    }
}

fn get_diff_elements(slice: &[i32]) -> usize {
    let mut set = HashSet::new();
    for e in slice {
        set.insert(e);
    }
    return set.len();
}