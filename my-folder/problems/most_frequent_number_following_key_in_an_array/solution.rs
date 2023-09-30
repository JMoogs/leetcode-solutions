use std::collections::HashMap;
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut key_flag = false;
        let mut occurs = HashMap::new();

        for num in nums {
            if key_flag {
                key_flag = false;

                let res = occurs.get(&num);
                match res {
                    None => occurs.insert(num, 1),
                    Some(v) => occurs.insert(num, v + 1),
                };
            }
            if num == key {
                key_flag = true;
            }
        }

        let mut max = (0, 0);
        for (k, v) in occurs.into_iter() {
            if v > max.1 {
                max = (k, v)
            }
        }

        max.0
    }
}
