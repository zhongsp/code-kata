//! https://leetcode.cn/problems/text-justification/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        // let mut ans: Vec<String> = vec![];
        let mut segs = vec![];
        // let mut buf: Vec<String> = vec![];

        let mut start = 0;
        let mut count = 0;
        let mut total_len = 0;

        for i in 0..words.len() {
            let new_len = if total_len == 0 {
                words[i].len()
            } else {
                total_len + words[i].len() + (count - 1)
            };

            if new_len as i32 <= max_width {
                total_len += words[i].len();
                count += 1;

                if i == words.len() - 1 {
                    segs.push((start, count));
                }
            } else {
                segs.push((start, count));

                start = i;
                count = 0;
                total_len = 0;
            }
        }

        println!("{:?}", segs);

        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string()
                ],
                16
            ),
            vec!["text".to_string()]
        );
    }
}
