// 28 Strstr Boyer-Moore Method
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() { return -1; }
        let src = haystack.as_bytes();
        let pat = needle.as_bytes();
        let mut bm_map = HashMap::new();
        for i in 0..pat.len() {
            bm_map.insert(pat[i], i);
        }

        let mut i = 0;
        while i <= src.len() - pat.len() {
            let mut found = true;
            for j in (0..pat.len()).rev() {
                if pat[j] != src[i + j] {
                    match bm_map.get(&src[i + j]) {
                        Some(loc) => { i += if j > loc + 1 { j - loc } else { 1 }; },
                        None => { i += j + 1; }
                    }
                    found = false;
                    break;
                }
            }
            if found {
                return i as i32;
            }
        }
        -1
    }
}
