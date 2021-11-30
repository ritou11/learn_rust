impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut a = head;
        let mut b = a.as_mut().unwrap().next.take();

        a.as_mut().unwrap().next = Solution::swap_pairs(b.as_mut().unwrap().next.take());
        b.as_mut().unwrap().next = a;
        b
    }
}
