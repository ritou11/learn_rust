use crate::{Solution, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&root).0
    }
    fn check(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
        if let Some(rrt) = node {
            let nd = rrt.borrow();
            if nd.left.is_none() && nd.right.is_none() { (true, nd.val, nd.val) }
            else {
                let (v, lmin, lmax) = Self::check(&nd.left);
                if !v { (false, i32::MIN, i32::MAX) }
                else {
                    let (v, rmin, rmax) = Self::check(&nd.right);
                    if !v || (nd.left.is_some() && lmax >= nd.val) || (nd.right.is_some() && rmin <= nd.val) { (false, i32::MIN, i32::MAX) }
                    else { (true, lmin.min(nd.val), rmax.max(nd.val)) }
                }
            }
        } else {
            (true, i32::MAX, i32::MIN)
        }
    }
}