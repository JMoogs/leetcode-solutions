impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;

        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx2 < n {
            if nums1[idx1] == 0 && idx1 >= m {
                std::mem::swap(&mut nums1[idx1], &mut nums2[idx2]);
                idx2 += 1;
                if idx2 == n {
                    break;
                }
            }

            if nums1[idx1] > nums2[idx2] {
                std::mem::swap(&mut nums1[idx1], &mut nums2[idx2]);
                let mut temp = idx2;
                while temp < nums2.len() - 1 && nums2[temp] > nums2[temp + 1] {
                    nums2.swap(temp, temp + 1);
                    temp += 1;
                }
            }

            idx1 += 1;
        }

    }
}