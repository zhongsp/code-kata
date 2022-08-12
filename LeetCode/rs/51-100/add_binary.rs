//! https://leetcode.cn/problems/add-binary/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let la = a.len();
        let lb = b.len();
        let lm = usize::max(la, lb);
        let mut ans = vec![0; lm];

        let mut carry = 0;
        for i in 0..lm {
            let va = if i + 1 <= la {
                a.as_bytes()[la - 1 - i]
            } else {
                b'0'
            };
            let vb = if i + 1 <= lb {
                b.as_bytes()[lb - 1 - i]
            } else {
                b'0'
            };

            match (va, vb, carry) {
                (b'0', b'1', 1) | (b'1', b'0', 1) | (b'1', b'1', 0) => {
                    ans[lm - 1 - i] = b'0';
                    carry = 1;
                }

                (b'1', b'1', 1) => {
                    ans[lm - 1 - i] = b'1';
                    carry = 1;
                }

                (b'0', b'0', 0) => {
                    ans[lm - 1 - i] = b'0';
                    carry = 0;
                }

                _ => {
                    ans[lm - 1 - i] = b'1';
                    carry = 0;
                }
            }
        }

        if carry == 1 {
            ans.insert(0, b'1');
        }

        String::from_utf8(ans).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            String::from("100")
        );
        assert_eq!(
            Solution::add_binary(String::from("1"), String::from("11")),
            String::from("100")
        );
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            String::from("10101")
        );
        assert_eq!(
            Solution::add_binary(String::from("0"), String::from("0")),
            String::from("0")
        );
        assert_eq!(
            Solution::add_binary(String::from("0"), String::from("1")),
            String::from("1")
        );
    }
}
