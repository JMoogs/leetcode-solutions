impl Solution {
pub fn min_max_difference(num: i32) -> i32 {
        let chars: Vec<char> = num.to_string().chars().collect();

        let max: i32 = {
            let mut s = String::with_capacity(chars.len());
            let mut replaced = None;
            for ch in chars.iter() {
                if ch != &'9' && replaced.is_none() {
                    replaced = Some(ch);
                    s.push('9');
                } else if replaced.is_some_and(|x| ch == x) {
                    s.push('9');
                } else {
                    s.push(*ch);
                }
            }

            s.parse().unwrap()
        };

        let min: i32 = {
            let mut a = chars.iter();
            let num = a.next().unwrap();
            let s = a.collect::<String>().replace(*num, "0");
            if s.is_empty() {
                0
            } else {
                s.parse().unwrap()
            }
        };

        println!("{}, {}", max, min);

        max - min
    }

}