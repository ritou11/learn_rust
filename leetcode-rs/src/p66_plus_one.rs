// 66 Plus One
use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut ex = true;
        let mut i = digits.len();
        while i > 0 {
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                ex = false;
                break;
            }
        }
        if ex {
            digits.insert(0, 1);
        }
        digits
    }
}
