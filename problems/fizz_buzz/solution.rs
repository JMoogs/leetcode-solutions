impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut v = Vec::with_capacity(n as usize);
        for i in 1..(n + 1) {
            if i % 15 == 0 {
                v.push(String::from("FizzBuzz"))
            } else if i % 5 == 0 {
                v.push(String::from("Buzz"))
            } else if i % 3 == 0 {
                v.push(String::from("Fizz"))
            } else {
                v.push(i.to_string())
            }
        }
        return v;
    }

}