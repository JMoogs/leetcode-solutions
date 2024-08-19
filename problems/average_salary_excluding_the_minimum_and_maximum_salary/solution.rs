impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        // Get sum

        let mut total = 0;

        let divisor = salary.len() - 2;

        // the lowest has to be ignored initially but must be bigger than 0
        let mut lowest = i32::MAX;

        let mut highest = i32::MIN;

        for sal in salary {
            if sal < lowest {
                lowest = sal;
            }

            if sal > highest {
                highest = sal;
            }


            total += sal
        }

        total -= highest;

        total -= lowest;

        total as f64 / divisor as f64
    }
}