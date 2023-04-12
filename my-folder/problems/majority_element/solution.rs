use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
    
        let mut amount = HashMap::new();

        for num in nums {
            if amount.contains_key(&num) {
                amount.insert(num, amount.get(&num).unwrap() + 1);
            } else {
                amount.insert(num, 1);
            }
        }

       let mut max = 0;
       let mut maxv = -1;
       for (k, v) in amount.into_iter() {
            if v > max {
                maxv = k;
                max = v;
            }
       }

       return maxv
    }
}