use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        if s.len() <= 1 {
            return String::from("");
        }
        let bytes = s.into_bytes();
        let mut max_len = 0;
        let mut max_pos = 0;

        let mut know: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
        for k in 1..bytes.len() {
            let before = know.get(&bytes[k - 1]);
            if before.is_some() {
                let lst = before.unwrap().clone();
                let ctails = know.entry(bytes[k]).or_insert(Vec::<(usize, usize)>::new());
                for pair in lst {
                    if pair.0 + pair.1 != k && bytes[pair.0 + pair.1] == bytes[k] {
                        ctails.push((pair.0, pair.1 + 1));
                        if pair.1 + 1 > max_len {
                            max_len = pair.1 + 1;
                            max_pos = pair.0;
                        }
                    }
                }
                ctails.push((k, 1));
            } else {
                let ctails = know.entry(bytes[k]).or_insert(Vec::<(usize, usize)>::new());
                ctails.push((k, 1));
            }
        }

        if max_len > 0 {
            String::from_utf8(bytes[max_pos..max_pos + max_len].to_vec()).unwrap()
        } else {
            String::from("")
        }
    }
}
pub fn longest_dup_substring(s: String) -> String {
    if s.len() <= 1 {
        return String::from("");
    }
    let mut tails: Vec<(usize, Vec<usize>)> = Vec::new();
    tails.push((0, vec![]));
    let bytes = s.into_bytes();
    let mut max_len = 0;
    let mut max_pos = 0;
    for k in 1..bytes.len() {
        let c = bytes[k];
        let mut lst: Vec<usize> = vec![];
        let i = tails[k - 1].0;
        if i == 0 {
            for p in 0..k {
                if bytes[p] == c {
                    lst.push(p);
                }
            }
        } else {
            for p in &tails[k - 1].1 {
                if bytes[p + i] == c {
                    lst.push(*p);
                }
            }
        }
        if lst.len() > 0 {
            if i + 1 > max_len {
                max_len = i + 1;
                max_pos = lst[0];
            }
            tails.push((i + 1, lst));
        } else {
            for p in 0..k {
                if bytes[p] == c {
                    lst.push(p);
                }
            }
            if lst.len() > 0 {
                tails.push((1, lst));
            } else {
                tails.push((0, lst));
            }
        }
    }
    // println!("{:?}", tails);
    if max_len > 0 {
        String::from_utf8(bytes[max_pos..max_pos + max_len].to_vec()).unwrap()
    } else {
        String::from("")
    }
}
