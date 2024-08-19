impl Solution {
pub fn longest_common_prefix(strs: Vec<String>) -> String {

    let mut common: String = String::from("");

    let mut minl: usize = usize::MAX;
    let mut shortest = &String::new();

    for string in &strs {
        if string.len() < minl {
            minl = string.len();
            shortest = string;
        }
    }


    if strs.len() == 1 || shortest.len() == 0 {
        return shortest.to_string()
    }

    let mut idx = 0;
    while idx < shortest.len() {
        let mut added = false;
        let mut letter = "";
        for string in &strs {
            if string[idx..=idx] != shortest[idx..=idx] {

                return common
            } else {
                letter = &string[idx..=idx];
            }
            // if !added {
            //     common.push_str(&string[idx..=idx]);
            //     added = true
            // }

        }
        common.push_str(letter);
        idx += 1;
    }

    common
    
    }
}