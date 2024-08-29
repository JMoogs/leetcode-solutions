impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = Vec::new();
        for c in s.chars() {
            if is_vowel(&c) {
                vowels.push(c);
            }
        }

        s.chars()
            .map(|x| {
                if is_vowel(&x) {
                    vowels.pop().unwrap()
                } else {
                    x
                }
            })
            .collect()
    }
}

#[inline(always)]
fn is_vowel(c: &char) -> bool {
    ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c)
}
