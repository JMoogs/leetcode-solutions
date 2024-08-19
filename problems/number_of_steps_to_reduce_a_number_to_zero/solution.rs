impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut steps: i32 = 0;
        let mut n = num;
        
        while n != 0 {
            while n % 2 == 0 {
                n = n / 2;
                steps += 1;
            }
            
            n -= 1;
            steps += 1;
        }
        /*
        if n % 2 == 0 {
            n = n / 2;
            steps += 1;
        }
        while n != 0 {
            n -= 1;
            steps += 1;
            if n == 0 {
                break;
            }
            n = n / 2;
            steps += 1;
        }*/
        
        steps
    }
}