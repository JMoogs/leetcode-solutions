impl Solution {
pub fn count_bits(n: i32) -> Vec<i32> {
  let mut arr: Vec<i32> = vec![];

  for mut i in 0..=n {


    let mut ones = 0;
    while i != 0 {
      i = i & (i - 1);
      ones += 1
    }

    arr.push(ones)

  }

  arr
}


}