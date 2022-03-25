//! https://leetcode-cn.com/problems/substring-with-concatenation-of-all-words/

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let words_len = words.len();
        let words_len_total = words[0].len() * words_len;
        let mut words_map = HashMap::new();
        for word in words.iter() {
            if let Some(x) = words_map.get_mut(word) {
                *x += 1;
            } else {
                words_map.insert(word, 1);
            }
        }

        let mut ans = vec![];

        let step = words[0].len();
        for start in 0..step {
            let mut start = start;
            loop {
                if start >= s.len() {
                    break;
                }
                if start + words_len_total > s.len() {
                    break;
                }

                let mut cur = start;
                let mut map = words_map.clone();

                for i in 0..words_len {
                    let cur_str = &s[cur..(cur + step)].to_string();

                    if let Some(x) = map.get_mut(cur_str) {
                        if *x != 0 {
                            *x -= 1;

                            if i == words_len - 1 {
                                ans.push(start as i32);
                            } else {
                                cur += step;
                                continue;
                            }
                        }
                    }

                    start += step;
                    break;
                }

                // 优化点：
                // 如果这次成功匹配出一个解，下次可以直接跳到 start + words_len_total
                // 并判断此处的单词是否为 start 处的单词
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_substring0() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("foo"), String::from("bar")]
            ),
            vec![0, 9]
        )
    }

    #[test]
    fn test_find_substring1() {
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word")
                ]
            ),
            vec![]
        )
    }

    #[test]
    fn test_find_substring2() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoofoobarthefoobarman"),
                vec![
                    String::from("bar"),
                    String::from("foo"),
                    String::from("the")
                ]
            ),
            vec![6, 9, 12]
        )
    }

    #[test]
    fn test_find_substring3() {
        assert_eq!(
            Solution::find_substring(
                String::from("foo"),
                vec![String::from("o"), String::from("o")]
            ),
            vec![1]
        )
    }

    #[test]
    fn test_find_substring4() {
        assert_eq!(
            Solution::find_substring(
                String::from("a"),
                vec![String::from("o"), String::from("o")]
            ),
            vec![]
        )
    }
}
