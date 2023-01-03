//! https://leetcode.cn/problems/word-search/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let w = board[0].len();
        let h = board.len();
        let mut t = vec![vec![false; w]; h];

        for j in 0..h {
            for i in 0..w {
                if Self::check(i, j, &board, &word[0..], &mut t) {
                    return true;
                }
            }
        }

        false
    }

    fn check(
        i: usize,
        j: usize,
        board: &Vec<Vec<char>>,
        word: &str,
        t: &mut Vec<Vec<bool>>,
    ) -> bool {
        if t[j][i] {
            return false;
        }

        if word.chars().nth(0) == Some(board[j][i]) {
            if word.len() == 1 {
                return true;
            }

            t[j][i] = true;

            let w = board[0].len();
            let h = board.len();
            if j > 0 && Self::check(i, j - 1, board, &word[1..], t) {
                return true;
            }
            if j + 1 < h && Self::check(i, j + 1, board, &word[1..], t) {
                return true;
            }
            if i > 0 && Self::check(i - 1, j, board, &word[1..], t) {
                return true;
            }
            if i + 1 < w && Self::check(i + 1, j, board, &word[1..], t) {
                return true;
            }

            t[j][i] = false;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");

        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn t1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("SEE");

        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn t2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");

        assert_eq!(Solution::exist(board, word), false);
    }
}
