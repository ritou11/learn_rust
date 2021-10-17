// p437 Path Sum III
use crate::Solution;
use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Solution::path_sum_list(&root, &vec![target_sum], target_sum)
    }

    fn path_sum_list(root: &Option<Rc<RefCell<TreeNode>>>, target_sums: &Vec<i32>, restart_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                let rt = r.borrow();
                let mut nlist: Vec<i32> = vec![restart_sum];
                let mut stop: i32 = 0;
                for t in target_sums {
                    if *t == rt.val {
                        stop += 1;
                    }
                    nlist.push(*t - rt.val);
                }
                stop + Solution::path_sum_list(&rt.left, &nlist, restart_sum) + Solution::path_sum_list(&rt.right, &nlist, restart_sum)
            }, 
        }
    }
}
