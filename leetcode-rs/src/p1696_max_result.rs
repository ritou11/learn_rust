use crate::Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut d = Vec::new();
        for i in 0..n {
            d.push((i32::MIN, i));
        }
        d[n - 1] = (nums[n - 1], n - 1);
        let mut r = n - 1;
        let mut curr_max = d[n - 1].0;
        for i in (0..n - 1).rev() {
            d[i].0 = curr_max + nums[i];
            curr_max = d[i].0;
            let mut j = i;
            while j <= r {
                if d[j].0 < d[i].0 || d[j].1 >= i + k as usize {
                    d.swap(j, r);
                    r -= 1;
                } else {
                    curr_max = curr_max.max(d[j].0);
                    j += 1;
                }
            }
        }
        d[0].0
    }
}
