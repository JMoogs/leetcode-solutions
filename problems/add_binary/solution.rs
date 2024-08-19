impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
    let a_b = u128::from_str_radix(&a, 2).unwrap();
    let b_b = u128::from_str_radix(&b, 2).unwrap();

    let sum = a_b + b_b;
    format!("{sum:b}")
    }
}