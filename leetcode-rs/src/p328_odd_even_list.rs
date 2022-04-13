use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut odd = Box::new(ListNode::new(-1));
        let mut cur_odd = &mut odd;
        let mut even = Box::new(ListNode::new(-1));
        let mut cur_even = &mut even;
        let mut is_odd = true;
        while curr.is_some() {
            if is_odd {
                cur_odd.next = curr;
                cur_odd = cur_odd.next.as_mut()?;
                curr = cur_odd.next.take();
            } else {
                cur_even.next = curr;
                cur_even = cur_even.next.as_mut()?;
                curr = cur_even.next.take();
            }
            is_odd = !is_odd;
        }
        cur_odd.next = even.next;
        return odd.next;
    }
}