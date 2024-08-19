impl Solution {
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // find current max
    let mut max = i32::MIN;
    for c in &candies {
        if c > &max {
            max = *c
        }
    }       
    
    let mut ret: Vec<bool> = vec![];

    // Return true if >= max
    for c in candies {
        if (c + extra_candies) >= max {
            ret.push(true)
        } else {
            ret.push(false)
        }
    }
    ret
}
}