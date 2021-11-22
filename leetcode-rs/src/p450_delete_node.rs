use crate::Solution;
use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn delete_node(root: Node, key: i32) -> Node {
        Solution::delete(&root, key)
    }

    pub fn delete(root: &Node, key:i32) -> Node {
        if root.is_none() { return root.clone(); }
        if let Some(node) = root {
            let val = node.borrow().val;
            match val.cmp(&key) {
                Ordering::Greater => {
                    let tmp = Solution::delete(&node.borrow().left, key);
                    node.borrow_mut().left = tmp;
                },
                Ordering::Less => {
                    let tmp = Solution::delete(&node.borrow().right, key);
                    node.borrow_mut().right = tmp;
                },
                Ordering::Equal => {
                    if node.borrow().left.is_none() {
                        return node.borrow().right.clone();
                    }
                    if node.borrow().right.is_none() {
                        return node.borrow().left.clone();
                    }
                    let largest = Solution::find_largest(&node.borrow().left);
                    let tmp = Solution::delete(&node.borrow().left, largest);
                    node.borrow_mut().val = largest;
                    node.borrow_mut().left = tmp;
                },
    
            }
        }
        root.clone()
    }

    pub fn find_largest(node: &Node) -> i32 {
        if node.is_some() && node.as_ref().unwrap().borrow().right.is_some() {
            Solution::find_largest(&node.as_ref().unwrap().borrow().right)
        } else if node.is_some() {
            node.as_ref().unwrap().borrow().val
        } else {
            -1
        }
    }
}
