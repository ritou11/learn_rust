impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        let mut curr = &head;
        while curr.is_some() {
            let inner = curr.as_ref().unwrap();
            res = inner.val + (res << 1);
            curr = &inner.next;
        }
        res
    }
}