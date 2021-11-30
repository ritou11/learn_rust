impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut i = 0;
        let mut pairs = Vec::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(i);
                },
                _ => {
                    if stack.len() > 0 {
                        let last = stack.pop().unwrap();
                        pairs.push((last, i));
                    } else {
                        stack.clear();
                    }
                }
            }
            i += 1;
        }
        pairs.sort();
        
        if pairs.len() == 0 { return 0; }
        let mut acc = pairs[0].1 - pairs[0].0 + 1;
        let mut res = acc;
        let mut last = pairs[0].1;
        for i in 1..pairs.len() {
            if pairs[i].0 <= last { continue; }
            if pairs[i].0 != last + 1 {
                acc = 0;
            }
            acc += pairs[i].1 - pairs[i].0 + 1;
            res = res.max(acc);
            last = pairs[i].1;
        }
        res as i32
    }
}
