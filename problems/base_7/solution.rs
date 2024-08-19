impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let negative = num.is_negative();
        let mut num = num.abs() as u32;

        let mut n = String::new();
        if num == 0 {
            return "0".into();
        }

        while num != 0 {
            n.push(char::from_digit(num % 7, 10).unwrap());
            num /= 7;
        }

        if negative {
            n.push('-');
        }

        n.chars().rev().collect()
    }

}