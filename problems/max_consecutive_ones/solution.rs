impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_1 = 0;
        let mut cur_1 = 0;

        for num in nums {
            if num == 1 {
                cur_1 += 1;
                if max_1 < cur_1 {
                    max_1 = cur_1;
                }
            } else {
                cur_1 = 0;
            }
        }
        max_1
    }
}
