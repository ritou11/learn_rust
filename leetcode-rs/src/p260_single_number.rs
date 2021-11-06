impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut r = 0;
        for num in &nums {
            r ^= *num;
        }
        r ^= 0;
        let mut cnt = 0;
        let s1 = r;

        while r & 1 == 0 {
            cnt += 1;
            r = r >> 1;
        }

        let mut s2 = 0;
        for num in &nums {
            if (*num >> cnt) & 1 == 1 {
                s2 ^= *num;
            }
        }
        s2 ^= 0;

        res.push(s2);
        res.push(s1 ^ s2);
        
        res
    }
}
