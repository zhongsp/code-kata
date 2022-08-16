//! https://leetcode.cn/problems/text-justification/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut words = words;
        let max_width = max_width as usize;

        let mut segs = vec![];
        let mut start = 0;
        let mut count = 0;
        let mut total_len = 0;

        for i in 0..words.len() {
            let new_len = if total_len == 0 {
                words[i].len()
            } else {
                total_len + words[i].len() + count
            };

            if new_len <= max_width {
                total_len += words[i].len();
                count += 1;
            } else {
                segs.push((start, count, total_len));

                start = i;
                count = 1;
                total_len = words[i].len();
            }

            if i == words.len() - 1 {
                segs.push((start, count, total_len));
            }
        }

        let mut ans: Vec<String> = vec![];
        let segs_len = segs.len();

        for (index, (start, count, total_len)) in segs.into_iter().enumerate() {
            if index == segs_len - 1 {
                let x = (&words[start..start + count]).to_owned().join(" ");
                let x_len = x.len();
                ans.push(x + &" ".repeat(max_width - x_len));
            } else {
                if count == 1 {
                    let x_len = words[start].len();
                    ans.push(words[start].to_owned() + &" ".repeat(max_width - x_len));
                } else {
                    if (max_width - total_len) % (count - 1) != 0 {
                        for k in 0..(max_width - total_len) % (count - 1) {
                            words[start + k] += &" ";
                        }
                    }

                    ans.push(
                        words[start..start + count]
                            .to_owned()
                            .join(&" ".repeat((max_width - total_len) / (count - 1))),
                    )
                }
            }
        }

        ans
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
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string()
            ]
        );
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string()
                ],
                16
            ),
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string()
            ]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                    "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string()
                ],
                20
            ),
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string()
            ]
        );
    }
}
