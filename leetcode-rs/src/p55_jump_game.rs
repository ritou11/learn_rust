impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut nearest = n - 1;
        for j in (0..n).rev() {
            if j + nums[j] as usize >= nearest {
                nearest = j;
            }
        }
        nearest == 0
    }
}
