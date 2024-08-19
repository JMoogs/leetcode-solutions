impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut start_at_owned = None;
        let mut max_stretch = 0;
        let mut current_stretch = 0;
        for sq in forts {
            match start_at_owned {
                None => {
                    if sq == 1 {
                        start_at_owned = Some(true);
                        current_stretch = 0;
                    } else if sq == -1 {
                        start_at_owned = Some(false);
                        current_stretch = 0;
                    } else {
                        continue;
                    }
                }
                Some(owned) => {
                    if sq == 0 {
                        current_stretch += 1;
                    } else if sq == 1 {
                        if owned {
                            current_stretch = 0;
                        } else {
                            max_stretch = max_stretch.max(current_stretch);
                            start_at_owned = Some(true);
                            current_stretch = 0;
                        }
                    } else if sq == -1 {
                        if !owned {
                            current_stretch = 0;
                        } else {
                            max_stretch = max_stretch.max(current_stretch);
                            start_at_owned = Some(false);
                            current_stretch = 0;
                        }
                    }
                }
            }
        }

        max_stretch
    }
}