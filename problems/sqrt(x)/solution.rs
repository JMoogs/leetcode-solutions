impl Solution {

pub fn my_sqrt(x: i32) -> i32 {
    for i in 0..=x {
        let sq = i.checked_mul(i);
        match sq {
            None => return i - 1,
            Some(v) => {
                if v > x {
                    return i - 1
                }
            }
        }

    }   
    return x    
}
}