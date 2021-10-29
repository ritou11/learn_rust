// 19 Remove Nth Node From End of List
use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut f = dummy.clone();
        let mut s = dummy.as_mut();
        for _ in 0..n {
            f = f.next.unwrap();
        }
        while f.next.is_some() {
            f = f.next.unwrap();
            s = s.next.as_mut().unwrap();
        }
        s.next = s.next.as_mut().unwrap().next.clone();
        dummy.next
    }
}
