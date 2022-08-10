use crate::{Solution, TreeNode};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut d: HashMap<i32, (Option<i32>, Option<i32>)> = HashMap::new();
        let mut c: HashSet<i32> = HashSet::new();
        for desc in descriptions.iter() {
            let org = d.entry(desc[0]).or_insert((None, None));
            let new = if desc[2] == 0 { (org.0, Some(desc[1])) } else { (Some(desc[1]), org.1) };
            d.insert(desc[0], new);
            c.insert(desc[1]);
        }
        let mut val = 0;
        for k in d.keys() {
            if !c.contains(k) {
                val = *k;
                break;
            }
        }
        Self::build(&d, val)
    }

    fn build(d: &HashMap<i32, (Option<i32>, Option<i32>)>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some((l, r)) = d.get(&val) {
            let mut node = TreeNode::new(val);
            if l.is_some() { node.left = Self::build(d, l.unwrap()); }
            if r.is_some() { node.right = Self::build(d, r.unwrap()); }
            Some(Rc::new(RefCell::new(node)))
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }

}