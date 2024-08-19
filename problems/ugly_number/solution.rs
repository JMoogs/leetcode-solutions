impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false
        } else if n == 1 {
            return true
        }

        loop {
            let mut div = false;

            if n % 2 == 0 {
                n /= 2;
                div = true;
            }

            if n % 3 == 0 {
                n /= 3;
                div = true;
            }

            if n % 5 == 0 {
                n /= 5;
                div = true;
            }

            if !div {
                return false
            }

            if n == 1 {
                return true
            }
        }

        unreachable!()
    }
}