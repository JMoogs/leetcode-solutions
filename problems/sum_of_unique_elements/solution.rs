use std::collections::HashSet;
impl Solution {
    pub fn sum_of_unique(mut nums: Vec<i32>) -> i32 {
        
        let mut seen = HashSet::new();
        let mut blacklisted = HashSet::new();


        for num in &nums {
            if seen.contains(num) {
                blacklisted.insert(num);
            } else {
                seen.insert(num);
            }

        }

        nums.iter().filter(|n| !blacklisted.contains(n)).sum()

    }
}