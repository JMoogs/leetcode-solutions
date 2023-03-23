impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        
        let veclen = nums.len();

        if veclen == 0 {
            return 0
        }

        if veclen == 1 {
            if nums[0] == val {
                nums.pop();
                return 0
            }
        }

        let mut last_idx = veclen - 1;
        let mut rm_counter = 0;
        let mut current_idx = 0;


        loop {

            // Add checks for vars here
            if current_idx > last_idx {
                break;
            }

            if veclen - 1 < current_idx {
                break;
            }

            if last_idx == 0 {
                break;
            }

            // If the element doesn't need to be removed continue
            if nums[current_idx] != val {
                current_idx += 1;
                continue
            }

            match nums[last_idx] == val {
                true => {
                    // Decrement counter and match again next loop.
                    rm_counter += 1;
                    last_idx -= 1;
                    // if last_idx == veclen -1 {
                    //     break;
                    // }
                    continue
                },
                false => {
                    // Swap them
                    nums.swap(current_idx, last_idx);
                    rm_counter += 1;
                    current_idx += 1;
                    last_idx -= 1;
                    continue;
                },
            }
        }

    
    for _ in 0..rm_counter {
        nums.pop();
    }

    if nums.len() == 1 {
        if nums[0] == val {
            rm_counter += 1;
            nums.pop();
        }
    }
    
    return (veclen - rm_counter) as i32



    }
}