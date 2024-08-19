impl Solution {
    pub fn is_happy(n: i32) -> bool {


    let sums = Vec::new();

    return name(n, sums);

    fn name(num: i32, mut seen: Vec<i32>) -> bool {
        if num == 1 {
            return true
        }
        let mut total: i32 = 0;
        let num_s = num.to_string();
        
        let sum = num_s.chars().map(|d| (d.to_digit(10).unwrap() as i32).pow(2));

        for v in sum {
            total += v
        }



        if seen.contains(&total) {
            return false;
        };

        seen.push(total);


        return name(total, seen);
        
        
    }

    }
}