impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut chs = s.chars();

        let mut max_0 = 0;
        let mut max_1 = 0;

        let mut cur_0 = 0;
        let mut cur_1 = 0;

        while let Some(c) = chs.next() {
            match c {
                '0' => {
                    cur_0 += 1;
                    cur_1 = 0;
                    if max_0 < cur_0 {
                        max_0 = cur_0;
                    }
                }
                '1' => {
                    cur_1 += 1;
                    cur_0 = 0;
                    if max_1 < cur_1 {
                        max_1 = cur_1;
                    }
                }
                _ => unreachable!(),
            }
        }

        max_1 > max_0
    }
}
