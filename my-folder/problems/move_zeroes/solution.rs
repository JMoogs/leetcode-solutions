impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let ilen = nums.len();
        nums.retain(|num| num != &0);
        let flen = nums.len();
        
        let lena = ilen - flen;
        for _ in 0..lena {
            nums.push(0);
        }
    }
}