// 451 Frequency Sort
use crate::Solution;
use std::collections::HashMap;
use std::cmp::Reverse;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut clist: Vec<(char, usize)> = Vec::new();
        for (key, value) in &map {
            clist.push((*key, *value));
        }
        clist.sort_by_key(|k| Reverse(k.1));
        let mut res = String::with_capacity(s.len());
        for tup in clist {
            for _ in 0..tup.1 {
                res.push(tup.0);
            }
        }
        res
    }
}
