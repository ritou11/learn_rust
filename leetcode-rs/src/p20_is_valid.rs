use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<usize> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(0);
                },
                ')' => {
                    if stack.len() <= 0 || stack.pop().unwrap() != 0 { return false; }
                },
                '[' => {
                    stack.push(1);
                },
                ']' => {
                    if stack.len() <= 0 || stack.pop().unwrap() != 1 { return false; }
                },
                '{' => {
                    stack.push(2);
                },
                '}' => {
                    if stack.len() <= 0 || stack.pop().unwrap() != 2 { return false; }
                },
                _ => {},
            }
        }
        stack.len() == 0
    }
}
