impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
    let z = x.leading_zeros();
    let x_b = format!("{:b}", x);
    let mut x_b: String = x_b.chars().rev().collect();
    for i in 0..z {
        x_b.push_str("0");
    }
    return u32::from_str_radix(&x_b, 2).unwrap();
    }
}