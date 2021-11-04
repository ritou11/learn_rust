use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::search(&root, 0)
    }
    
    pub fn search(node: &Option<Rc<RefCell<TreeNode>>>, curr: i32) -> i32 {
        let nd = node.as_ref().unwrap().borrow();
        let mut res = 0;
        if nd.left.is_none() && nd.right.is_none() {
            res = curr * nd.val;
        } else {
            if nd.left.is_some() {
                res += Solution::search(&nd.left, 1);
            }
            if nd.right.is_some() {
                res += Solution::search(&nd.right, 0);
            }
        }
        res
    }
}
