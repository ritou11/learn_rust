impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut flag = false;
        let mut min_r = 101;
        let mut min_i = 101;
        for r in (1..nums.len()).rev() {
            if nums[r - 1] < nums[r] {
                for i in r..nums.len() {
                    if nums[i] > nums[r - 1] && nums[i] < min_r {
                        min_r = nums[i];
                        min_i = i;
                    }
                }
                nums.swap(r - 1, min_i);
                nums[r..].sort();
                flag = true;
                break;
            }
        }
        if !flag {
            nums.reverse();
        }
    }
}
