struct CombinationIterator {
    stack: Vec<(usize, Vec<char>)>,
    curr: String,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        let mut stack = Vec::new();
        let mut curr = String::new();
        let chars: Vec<char> = characters.chars().collect();
        stack.push((0, chars.clone()));
        CombinationIterator {
            stack: stack,
            curr: curr,
        }
    }
    
    fn next(&self) -> String {
        while self.stack.len() > 0 {
            let top = self.stack.pop().unwrap();
            if top.1.len() == 0 {
                return self.curr.clone()
            } else if top.1.len() > top.0 {
                let mut next_nums = Vec::new();
                for i in &top.1 {
                    if *i != top.1[top.0] {
                        next_nums.push(*i);
                    }
                }
                if top.0 > 0 { self.curr.pop(); }
                self.curr.push(top.1[top.0]);
                stack.push((top.0 + 1, top.1.clone()));
                stack.push((0, next_nums));
            } else {
                self.curr.pop();
            }
        }
        String::new()
    }
    
    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */