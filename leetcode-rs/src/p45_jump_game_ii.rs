impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut far = 0;
        let mut end = 0;
        let mut res = 0;
        for i in 0..nums.len()-1 {
            far = far.max(i + nums[i] as usize);
            if i == end {
                res += 1;
                end = far;
            }
        }
        res
    }

    pub fn jump_slow(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut step: Vec<usize> = Vec::new();
        for _ in 0..n {
            step.push(n);
        }
        step[n-1] = 0;
        for i in (0..n-1).rev() {
            if i + nums[i] as usize >= n - 1 {
                step[i] = 1;
            } else {
                let mut min = n;
                for j in i+1..i+1+nums[i] as usize {
                    min = min.min(step[j]);
                }
                step[i] = min + 1;
            }
        }
        step[0] as i32
    }
}
