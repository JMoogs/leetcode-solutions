impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(nums.len());

        for n in nums.iter() {
            v.push(nums[*n as usize])
        }

        v
    }

}