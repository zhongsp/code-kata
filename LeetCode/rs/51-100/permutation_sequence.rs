//!https://leetcode.cn/problems/permutation-sequence/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut ans = vec![];
        let mut flag = vec![0; n as usize];
        flag.insert(0, 1);
        let fac_n = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

        let mut n = n;
        let mut k = k;
        while n > 0 {
            let group = (k - 1) / (fac_n[n as usize] / n); // 0 - 3

            let mut count = 0;
            for i in 0..flag.len() {
                let v = flag[i];
                if v == 0 {
                    if count == group {
                        flag[i] = 1;
                        ans.push(i as i32);
                        break;
                    } else {
                        count += 1;
                    }
                }
            }

            k = k - ((fac_n[n as usize] / n) * group);
            n -= 1;
        }

        ans.into_iter().map(|i| format!("{}", i)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_permutation0() {
        assert_eq!(Solution::get_permutation(3, 1), String::from("123"));
        assert_eq!(Solution::get_permutation(1, 1), String::from("1"));
    }
}
