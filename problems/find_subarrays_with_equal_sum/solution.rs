use std::collections::HashSet;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        let iter = nums.windows(2);
        for v in iter {
            if !set.insert(v[0] + v[1]) {
                return true;
            }
        }

        false
    }
}
