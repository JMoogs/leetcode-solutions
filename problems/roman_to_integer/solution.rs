impl Solution {
pub fn roman_to_int(s: String) -> i32 {
    let it = s.chars();

    let mut sum = 0;

    let mut previ = false;
    let mut prevx = false;
    let mut prevc = false;

    for c in it {
        let numeral = match c {
            'V' => {
                if previ {
                    previ = false;
                    3
                } else {
                    5
                }
            },
            'L' => {
                if prevx {
                    prevx = false;
                    30
                } else {
                    50
                }
            },
            'D' => {
                if prevc {
                    prevc = false;
                    300
                } else {
                    500
                }
            },
            'M' => {
                if prevc {
                    prevc = false;
                    800
                } else {
                    1000
                }
            },
            'I' => {
                previ = true;
                1
            },
            'X' => {
                prevx = true;
                if previ {
                    8
                } else {
                    10
                }
            },
            'C' => {
                prevc = true;
                if prevx {
                    prevx = false;
                    80
                } else {
                    100
                }
            }
            _ => unreachable!()
        };

        sum += numeral
    }

    sum
}


}