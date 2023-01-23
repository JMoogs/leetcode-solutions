impl Solution {
pub fn tribonacci(n: i32) -> i32 {
let mut a = 0;
let mut b = 1;
let mut c = 1;
let mut temp;

for _ in 0..n {
    temp = a;

    a = b;
    b = c;
    c = c + a + temp;
}
return a
}
}