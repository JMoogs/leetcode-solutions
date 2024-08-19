impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let EVEN_MASK = 0b_01_01_01_01_01;
        let ODD_MASK = 0b_10_10_10_10_10;

        vec![
            (n & EVEN_MASK).count_ones() as i32,
            (n & ODD_MASK).count_ones() as i32,
        ]
    }


}