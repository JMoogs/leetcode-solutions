impl Solution {
    pub fn distribute_candies(mut candy_type: Vec<i32>) -> i32 {
        let edible = candy_type.len()/2;
        candy_type.dedup();
        let mut typ = Vec::new();
        for c in candy_type {
            if !typ.contains(&c) {
                typ.push(c);
            }
        }

        if edible > typ.len() {
            return typ.len() as i32
        } else {
            return edible as i32
        }
    }
}