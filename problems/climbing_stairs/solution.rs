impl Solution {
pub fn climb_stairs(n: i32) -> i32 {

    fn fib(n: i32) -> i32 {
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
        return fib(n+1)
}


}