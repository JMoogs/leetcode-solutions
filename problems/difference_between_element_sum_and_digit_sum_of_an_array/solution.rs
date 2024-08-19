impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let element_sum: i32 = nums.iter().sum();

        let digit_sum: i32 = nums.into_iter().map(Solution::number_digit_sum).sum();

        return (element_sum - digit_sum).abs();
    }

    #[inline(always)]
    fn number_digit_sum(mut num: i32) -> i32 {
        let mut total = 0;
        loop {
            total += num % 10;
            num /= 10;

            if num == 0 {
                break;
            }
        }
        total
    }
}
