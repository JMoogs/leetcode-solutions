impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {

        fn factorial(num: u128) -> u128 {
        match num {
            0 | 1 => 1,
            _ => factorial(num - 1) * num
        }
    }

        let mut v = Vec::new();
        for i in 0..=row_index {

            let calc = factorial(row_index as u128) / (factorial(i as u128) * factorial(row_index as u128 - (i as u128)));

            v.push(calc as i32)
            
        }

        v
    }
}