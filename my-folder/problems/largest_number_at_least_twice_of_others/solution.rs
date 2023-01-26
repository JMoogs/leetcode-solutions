impl Solution {
pub fn dominant_index(nums: Vec<i32>) -> i32 {

    // Find the largest value and its index
    let mut max = i32::MIN;
    let mut idx: usize = 0; // Will always be set, must be initialized for second loop.
    for n in nums.iter().enumerate() {
        if n.1 > &max {
            max = *n.1;
            idx = n.0;
        }
    }     

    let nums = nums.into_iter().map(|x| x * 2 ); // Map to doubled values
    for n in nums.enumerate() { // Check doubled values
        if n.0 == idx {
            continue
        } else {
            if n.1 > max {
                return -1
            }
        }
    }

    idx as i32

}
}