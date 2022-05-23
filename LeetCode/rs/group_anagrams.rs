//! https://leetcode.cn/problems/group-anagrams/

use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut s = str.bytes().collect::<Vec<u8>>();
            s.sort();
            if let Some(val) = ans.get_mut(&s) {
                val.push(str);
            } else {
                ans.insert(s, vec![str]);
            }
        });

        ans.into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn group_anagrams0() {
        assert_eq!(
            Solution::group_anagrams(vec![String::from("a")]),
            vec![vec![String::from("a")]]
        );
    }

    #[test]
    fn group_anagrams1() {
        assert_eq!(
            Solution::group_anagrams(vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ]),
            vec![vec![String::from("a")]]
        );
    }

    #[test]
    fn group_anagrams2() {
        assert_eq!(
            Solution::group_anagrams(vec![String::from("")]),
            vec![vec![String::from("")]]
        );
    }
}
