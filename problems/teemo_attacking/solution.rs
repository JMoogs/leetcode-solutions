impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut t_dur = 0;
    for idx in 0..(time_series.len()) {
        if idx == time_series.len() - 1 {
            t_dur = t_dur + duration;
            break
        }
        let t_diff = time_series[idx+1] - time_series[idx];
        if t_diff > duration {
            t_dur = t_dur + duration;
        } else {
            t_dur = t_dur + t_diff
        }

    }
    t_dur
    }
}