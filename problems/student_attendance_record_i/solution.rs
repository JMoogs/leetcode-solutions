    impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absents = 0;
        let mut conseq_late = 0;
        let mut late_last = false;
        for c in s.chars() {
            match c {
                'A' => {
                    if absents == 1 {
                        return false;
                    } else {
                        absents += 1;
                        late_last = false;
                        conseq_late = 0;
                    }
                }
                'L' => {
                    // We always add one if there are none
                    if conseq_late == 0 {
                        conseq_late += 1;
                    }
                    if late_last {
                        conseq_late += 1;
                    }
                    if conseq_late == 3 {
                        return false;
                    }

                    late_last = true;
                }
                'P' => {
                    late_last = false;
                    conseq_late = 0;
                }
                _ => unreachable!(),
            }
        }

        return true;
    }
    }