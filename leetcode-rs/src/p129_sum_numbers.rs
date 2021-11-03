use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::search(&root, 0)
    }

    pub fn search(node: &Option<Rc<RefCell<TreeNode>>>, curr: i32) -> i32 {
        let nd = node.as_ref().unwrap().borrow();
        let mut res = 0;
        if nd.left.is_none() && nd.right.is_none() {
            res = curr * 10 + nd.val;
        } else {
            if nd.left.is_some() {
                res += Solution::search(&nd.left, curr * 10 + nd.val);
            }
            if nd.right.is_some() {
                res += Solution::search(&nd.right, curr * 10 + nd.val);
            }
        }
        res
    }
}
