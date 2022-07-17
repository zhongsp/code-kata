//! https://leetcode.cn/problems/unique-paths/

pub struct Node {
    right: u32,
    down: u32,
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans = 0;
        let max_right = (m - 1) as u32;
        let max_down = (n - 1) as u32;
        let mut nodes = vec![Node { right: 0, down: 0 }];

        if max_right == 0 && max_down == 0 {
          return 1;
        }

        loop {
            let len = nodes.len();
            if len == 0 {
                break;
            }

            for _ in 0..len {
                let node = nodes.remove(0);

                if node.right + 1 < max_right {
                    nodes.push(Node {
                        right: node.right + 1,
                        ..node
                    });
                } else if node.right + 1 == max_right {
                    ans += 1;
                }

                if node.down + 1 < max_down {
                    nodes.push(Node {
                        down: node.down + 1,
                        ..node
                    });
                } else if node.down + 1 == max_down {
                    ans += 1;
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
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::unique_paths(23, 12), 3);
    }
}
