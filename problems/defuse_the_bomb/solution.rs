impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            let mut v = Vec::new();
            v.resize(code.len(), 0);
            return v;
        }

        let mut ret_vec: Vec<i32> = Vec::with_capacity(code.len());

        for i in 0..code.len() {
            if k > 0 {
                let mut sum = 0;
                for j in 1..=k.abs() {
                    sum += code[(i + j as usize) % code.len()];
                }

                ret_vec.push(sum);
            } else {
                let mut sum = 0;
                for j in 1..=k.abs() {
                    let idx = if j as usize > i {
                        (code.len() + i) - j as usize
                    } else {
                        i - j as usize
                    };
                    sum += code[idx];
                }

                ret_vec.push(sum);
            }
        }

        ret_vec
    }
}
