impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let numbers: Vec<i32> = nums;
        let mut i = 0;
        let mut j = 0;
        
        for value in &numbers {
            for secondval in &numbers {
                if value + secondval == target && i != j {
                    ret.push(i);
                    ret.push(j);
                    return ret
                }
                j += 1;
            }
         i += 1;
         j = 0;
        }
    return vec![1,2]
    }
}