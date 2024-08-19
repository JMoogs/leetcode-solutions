impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum: i32 = 0;
        let mut sums: Vec<i32> = Vec::new();
        for i in nums {
            sum += i;
            sums.push(sum)
        }
        sums
    }
}