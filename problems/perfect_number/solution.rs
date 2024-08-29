impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num != 1 && get_divisors(num) == 2 * num
    }
}

fn get_divisors(num: i32) -> i32 {
    let mut acc = 0;
    for i in 1..((num as f64).sqrt() + 1.0) as i32 {
        if num % i == 0 {
            acc += i;
            acc += num / i;
        }
    }
    acc
}
