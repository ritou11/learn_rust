use crate::{Solution, TreeNode, ListNode};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p = &head;
        let mut n = 0;
        while let Some(curr) = p {
            n += 1;
            p = &curr.next;
        }
        let mut head = head;
        Solution::build(&mut head, n)
    }
    fn build(node: &mut Option<Box<ListNode>>, l: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l == 0 { return None; }
        let left = Solution::build(node, l / 2);
        if let Some(head) = node {
            let mut curr = TreeNode::new(head.val);
            curr.left = left;
            *node = head.next.take();
            curr.right = Solution::build(node, l - l / 2 - 1);
            Some(Rc::new(RefCell::new(curr)))
        }
        else {
            None
        }
    }
}