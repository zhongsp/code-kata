//! https://leetcode.cn/problems/simplify-path/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path = path + "/";
        let mut ans: Vec<String> = vec![];
        let mut part = vec![];

        for ch in path.bytes() {
            match ch {
                b'/' => {
                    let s = String::from_utf8(part.clone()).unwrap();

                    if s == "." || s.is_empty() {
                    } else if s == ".." {
                        if ans.len() > 0 {
                            ans.pop();
                        }
                    } else {
                        ans.push(s);
                    }

                    part.clear();
                }

                _ => {
                    part.push(ch);
                }
            }
        }

        String::from("/") + &ans.join("/")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a/././b/./../../c/")),
            String::from("/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a//b////c/d//././/..")),
            String::from("/a/b/c")
        );
    }
}
