use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums1 {
            add_or_increment(&mut map, num);
        }

        let mut ans = Vec::new();

        for num in nums2 {
            match map.get_mut(&num) {
                None => continue,
                Some(n) => {
                    if *n != 0 {
                        ans.push(num);
                        *n -= 1;
                    }
                }
            }
        }
        ans
    }
}

fn add_or_increment(map: &mut HashMap<i32, u16>, num: i32) {
    match map.get_mut(&num) {
        Some(num) => *num += 1,
        None => {
            map.insert(num, 1);
        }
    };
}
