impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut a = 0;
let mut b = 1;
let mut temp;

for _ in 0..n {
    temp = a;
    a = b;
    b = b + temp;
}
return a
    }
}