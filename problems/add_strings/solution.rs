impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {

        let leading_zs = (num1.len() as i32 - num2.len() as i32).abs();

        let mut zeros = String::with_capacity(leading_zs as usize);

        for _ in 0..leading_zs {
            zeros.push('0');
        }

        if num1.len() < num2.len() {
            num1 = zeros + &num1;
        } else if num1.len() > num2.len() {
            num2 = zeros + &num2;
        }

        let nums1rev = num1.chars().rev().collect::<Vec<char>>();
        let nums2rev = num2.chars().rev().collect::<Vec<char>>();


        let mut solution_rev: Vec<u32> = Vec::with_capacity(nums1rev.len() + 1);
        // Both vecs will be the same length due to adding leading 0s
        //
        let mut carry: u32 = 0;
        for i in 0..nums1rev.len() {
            let ans = nums1rev[i].to_digit(10).unwrap() + nums2rev[i].to_digit(10).unwrap() + carry;

            carry = 0;

            if ans < 10 {
                solution_rev.push(ans);
            } else {
                carry = 1;
                solution_rev.push(ans % 10);
            }

        }
        
        if carry == 1 {
            solution_rev.push(1);
        }

        return solution_rev.into_iter().rev().map(|n| n.to_string()).collect();

    }
}