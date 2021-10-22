// 69 Sqrt
use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 { return 1 };
        let mut r = x / 2 + 1;
        if r > 46340 { r = 46341; };
        let mut l = 0;
        while l + 1 < r {
            let m = (l + r) / 2;
            let m2 = m * m;
            if m2 < x {
                l = m;
            } else if m2 == x {
                return m;
            } else {
                r = m;
            }
        }
        l
    }
}
