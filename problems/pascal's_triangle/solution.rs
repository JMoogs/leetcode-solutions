impl Solution {
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {

    fn factorial(num: u128) -> u128 {
        match num {
            0 | 1 => 1,
            _ => factorial(num - 1) * num
        }
    }

    fn genrow(row_num: u128) -> Vec<i32> {
        let mut v = Vec::new();
        for i in 0..=row_num {

            let calc = factorial(row_num) / (factorial(i) * factorial(row_num - (i)));

            v.push(calc as i32)
            
        }

        v
    }

    let mut v = Vec::new();

    for row in 0..num_rows {
        v.push(genrow(row as u128));
    }

    v

}
}