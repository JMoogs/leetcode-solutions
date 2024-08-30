impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        if nums.is_empty() {
            return Vec::new();
        }
        let mut current_range = (nums[0], nums[0]);

        for num in &nums[1..] {
            if *num == current_range.1 + 1 {
                current_range.1 += 1;
            } else {
                if current_range.0 == current_range.1 {
                    ans.push(format!("{}", current_range.0))
                } else {
                    ans.push(format!("{}->{}", current_range.0, current_range.1));
                }
                current_range = (*num, *num);
            }
        }

        if current_range.0 == current_range.1 {
            ans.push(format!("{}", current_range.0))
        } else {
            ans.push(format!("{}->{}", current_range.0, current_range.1));
        };

        ans
    }

}