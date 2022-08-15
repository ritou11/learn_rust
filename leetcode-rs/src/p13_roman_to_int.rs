use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut last = 0;
        for c in s.chars().rev() {
            let d = match c {
                'I' => 1, 'V' => 5, 'X' => 10,
                'L' => 50, 'C' => 100, 'D' => 500,
                'M' => 1000, _ => 0,
            };
            if d < last {
                res -= d;
            } else {
                res += d
            }
            last = d;
        }
        res
    }
}