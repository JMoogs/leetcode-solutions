impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        return (left..(right + 1))
            .filter(|x| is_self_dividing(*x))
            .collect();
    }
}

fn is_self_dividing(num: i32) -> bool {
    let digits = get_num_digits(num);
    if digits.contains(&0) {
        return false;
    }

    return digits.into_iter().all(|x| num % x == 0);
}

fn get_num_digits(mut num: i32) -> Vec<i32> {
    let mut v = Vec::new();
    while num != 0 {
        v.push(num % 10);
        num /= 10;
    }

    v
}
