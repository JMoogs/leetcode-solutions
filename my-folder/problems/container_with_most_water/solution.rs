impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let max_idx = height.len();
        let mut start = 0;
        let mut end = max_idx - 1;
        let mut max_area = height[start].min(height[end]) * (end - start) as i32;

        while start < end {
            if height[start] > height[end] {
                end -= 1;
            } else {
                start += 1;
            }
            max_area = max_area.max(height[start].min(height[end]) * (end - start) as i32)
        }

        max_area
    }

}