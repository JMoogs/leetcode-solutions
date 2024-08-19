impl Solution {
pub fn max_profit(prices: Vec<i32>) -> i32 {
  
  let (mut min, mut max) = (i32::MAX, i32::MIN);

  for price in prices {
    max = max.max(price - min);
    min = min.min(price);
  }

    if max < 0 {
        max = 0
    }
  max
}
}
