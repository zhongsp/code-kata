//! https://leetcode.cn/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // f(i, j):
        // i: first i of word1; j: first j of word2
        // least steps from word1[..=i] to word2[..=j]
        let mut f: Vec<Vec<usize>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        for i in 0..word1.len() + 1 {
            f[i][0] = i;
        }
        for j in 0..word2.len() + 1 {
            f[0][j] = j;
        }

        for i in 1..word1.len() + 1 {
            for j in 1..word2.len() + 1 {
                f[i][j] = 1 + usize::min(usize::min(f[i][j - 1], f[i - 1][j]), f[i - 1][j - 1]);

                if word1[i - 1..=i - 1] == word2[j - 1..=j - 1] {
                    f[i][j] = usize::min(f[i][j], f[i - 1][j - 1]);
                }
            }
        }

        f[word1.len()][word2.len()] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
