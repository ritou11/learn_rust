use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        if lists.len() == 0 { return None; }
        while lists.len() > 1 {
            let mut res: Vec<Option<Box<ListNode>>> = Vec::new();
            for i in 0..lists.len() / 2 {
                res.push(Solution::merge_2_lists(lists[i * 2].clone(), lists[i * 2 + 1].clone()));
            }
            if lists.len() % 2 == 1 {
                res.push(lists[lists.len() - 1].clone());
            }
            lists = res;
        }
        lists[0].clone()
    }

    pub fn merge_2_lists(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = dummy.as_mut();
        let mut h1 = a;
        let mut h2 = b;
        while (let Some(h1n) = h1) && h2.is_some() {
            let h1n = h1.clone().unwrap();
            let h2n = h2.clone().unwrap();
            if h1n.val < h2n.val {
                curr.next = h1;
                curr = curr.next.as_mut().unwrap();
                h1 = h1n.next;
            } else {
                curr.next = h2;
                curr = curr.next.as_mut().unwrap();
                h2 = h2n.next;
            }
        }
        while h1.is_some() {
            curr.next = h1;
            curr = curr.next.as_mut().unwrap();
            h1 = h1.unwrap().next;
        }
        while h2.is_some() {
            curr.next = h2;
            curr = curr.next.as_mut().unwrap();
            h2 = h2.unwrap().next;
        }
        dummy.next
    }
}
