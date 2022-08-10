use crate::{Solution, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::build(&nums, 0, nums.len());
    }

    fn build(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right { None }
        else if left + 1 == right { Some(Rc::new(RefCell::new(TreeNode::new(nums[left])))) }
        else {
            let m = (left + right) / 2;
            let mut node = TreeNode::new(nums[m]);
            node.left = Solution::build(nums, left, m);
            node.right = Solution::build(nums, m + 1, right);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}