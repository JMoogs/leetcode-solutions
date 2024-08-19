
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut possible = Vec::with_capacity(words.len());

        let top_row: Vec<char> = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
        let middle_row: Vec<char> = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
        let bottom_row: Vec<char> = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'];

        'outer: for word in words {
            let mut row = 0;
                // 1 is top_row
                // 2 is middle_row
                // 3 is bottom_row
            for c in word.to_ascii_lowercase().chars() {
                let current = if top_row.contains(&c) {
                    1
                } else if middle_row.contains(&c) {
                    2
                } else {
                    3
                };

                if row == 0 {
                    row = current
                } else {
                    if row != current {
                        continue 'outer;
                    }
                }
            }

            possible.push(word);
        }

        possible
    }
}

