impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut current = String::from("1");
        if n == 1 {
            return current;
        }

        for _ in 0..(n - 1) {
            current = runlength_encode(&current);
        }

        return current;
    }
}

fn runlength_encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();

    let mut res = String::new();
    let mut current_char = chars.next().unwrap();
    let mut ch_count = 1;

    while let Some(ch) = chars.next() {
        if ch == current_char {
            ch_count += 1;
        } else {
            res.push_str(&ch_count.to_string());
            res.push(current_char);
            current_char = ch;
            ch_count = 1;
        }
    }

    res.push_str(&ch_count.to_string());
    res.push(current_char);
    res
}
