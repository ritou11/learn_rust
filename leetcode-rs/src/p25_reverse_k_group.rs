use crate::Solution;
use crate::ListNode;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 || head.is_none() {
            return head;
        }
        let mut n = 0;
        let mut curr = &head;
        while curr.is_some() {
            n += 1;
            if n >= k { break; }
            curr = &curr.as_ref().unwrap().next;
        }
        if n < k { return head; }

        let mut stack = Vec::new();
        stack.push(head);
        for _ in 1..k {
            let tail = stack.len() - 1;
            let nx = stack[tail].as_mut().unwrap().next.take();
            stack.push(nx);
        }

        let mut tail = stack.pop().unwrap();
        let mut nd = tail.clone();
        let mut curr = &mut tail;
        for _ in (1..k as usize).rev() {
            let node = stack.pop().unwrap();
            curr.as_mut().unwrap().next = node;
            curr = &mut curr.as_mut().unwrap().next;
        }
        curr.as_mut().unwrap().next = Solution::reverse_k_group(nd.as_mut().unwrap().next.take(), k);
        tail
    }
}
