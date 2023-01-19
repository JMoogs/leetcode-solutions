impl Solution {
pub fn is_valid(s: String) -> bool {

    let mut openN = 0;
    let mut openS = 0;
    let mut openC = 0;

    let mut latest = vec![]; // 1 = Normal, 2 = Sqaure, 3 = Curly

    for c in s.chars() {
        match c {
            '(' => {
                openN += 1;
                latest.push(1);
            },
            ')' => {
                openN -= 1;
                if latest.last() != Some(&1) {
                    return false
                }
                latest.pop();
            },
            '[' => {
                openS += 1;
                latest.push(2);
            },
            ']' => {
                openS -= 1;
                if latest.last() != Some(&2) {
                    return false
                }
                latest.pop();
            },
            '{' => {
                openC += 1;
                latest.push(3);
            },
            '}' => {
                openC -= 1;
                if latest.last() != Some(&3) {
                    return false
                }
                latest.pop();
            },
        _ => unreachable!()
        }
    }
    if !(openC == 0 && openN == 0 && openS == 0) {
        return  false;
    }
    true
}
}