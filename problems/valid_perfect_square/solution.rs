impl Solution {
pub fn is_perfect_square(num: i32) -> bool {
    if num == 1 || num == 4 || num == 9 || num == 16 {return true}
    
    let start = num / 5;

    for i in 0..=start {
        let sq = i.checked_mul(i);
        if let Some(v) = sq {
            if v == num {
                return  true
            }
        }
    }

    false
}
}