//! https://leetcode.cn/problems/rotate-image/

struct Solution;

type Point = (usize, usize);

#[allow(dead_code)]
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix[0].len();

        for i in 0..len / 2 {
            let tl: Point = (i, i);
            let br = (len - 1 - i, len - 1 - i);

            for j in i..br.0 {
                let start = (j, i);
                let mut cur = start;
                let mut vcur = matrix[cur.1][cur.0];
                loop {
                    let mut tar = Solution::next_pos(tl, br, cur);
                    for _ in 1..(br.0 - tl.0) {
                        tar = Solution::next_pos(tl, br, tar);
                    }
                    let vtar = matrix[tar.1][tar.0];
                    matrix[tar.1][tar.0] = vcur;

                    if tar == start {
                        break;
                    } else {
                        cur = tar;
                        vcur = vtar;
                    }
                }
            }
        }
    }

    fn get_br(x: usize, len: usize) -> Point {
        (len - 1 - x, len - 1 - x)
    }

    pub fn next_pos(tl: Point, br: Point, cur: Point) -> Point {
        let x: usize;
        let y: usize;

        if cur == tl || cur.0 > tl.0 && cur.1 < br.1 {
            x = if cur.0 == br.0 { br.0 } else { cur.0 + 1 };
            y = if cur.0 == br.0 { cur.1 + 1 } else { cur.1 };
        } else {
            x = if cur.0 == tl.0 { tl.0 } else { cur.0 - 1 };
            y = if cur.0 == tl.0 { cur.1 - 1 } else { cur.1 };
        }

        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn rotate0() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn rotate1() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn next_pos0() {
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (0, 0)), (1, 0));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (1, 0)), (2, 0));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (2, 0)), (2, 1));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (2, 1)), (2, 2));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (2, 2)), (1, 2));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (1, 2)), (0, 2));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (0, 2)), (0, 1));
        assert_eq!(Solution::next_pos((0, 0), (2, 2), (0, 1)), (0, 0));
    }
}
