use crate::ListNode;
use crate::Solution;
use std::mem;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        if lists.len() == 0 {
            return None;
        }
        while lists.len() > 1 {
            let mut res: Vec<Option<Box<ListNode>>> = Vec::new();
            for i in 0..lists.len() / 2 {
                res.push(Solution::merge_2_lists(
                    lists[i * 2].clone(),
                    lists[i * 2 + 1].clone(),
                ));
            }
            if lists.len() % 2 == 1 {
                res.push(lists[lists.len() - 1].clone());
            }
            lists = res;
        }
        lists[0].clone()
    }

    pub fn merge_2_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val > n2.val {
                    mem::swap(&mut n1, &mut n2);
                }
                Some(Box::new(ListNode {
                    val: n1.val,
                    next: Solution::merge_2_lists(n1.next, Some(n2)),
                }))
            }
        }
    }
}
