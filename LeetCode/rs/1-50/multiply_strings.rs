//! https://leetcode.cn/problems/multiply-strings/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return String::from("0");
        }

        let mut x;
        let mut y;
        if num1.len() >= num2.len() {
            x = num1;
            y = num2;
        } else {
            x = num2;
            y = num1;
        };

        let mut ans = String::from("0");

        let mut n = 0;
        loop {
            if y.len() == 0 {
                break;
            }

            let mut d = y.pop().unwrap().to_digit(10).unwrap();
            x.push_str(&("0".repeat(n)));
            n = 1;
            while d > 0 {
                d -= 1;
                ans = Self::add(&x, &ans);
            }
        }

        ans
    }

    pub fn add(x: &String, y: &String) -> String {
        let len = usize::max(x.len(), y.len());
        let mut ans = String::new();
        let mut carry = 0;
        for i in 0..len {
            let xx = if i >= x.len() {
                0
            } else {
                x.chars()
                    .nth(x.len() - 1 - i)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
            };
            let yy = if i >= y.len() {
                0
            } else {
                y.chars()
                    .nth(y.len() - 1 - i)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
            };
            ans.insert(0, char::from_digit((xx + yy + carry) % 10, 10).unwrap());
            carry = (xx + yy + carry) / 10;
        }

        if carry > 0 {
            ans.insert(0, char::from_digit(carry, 10).unwrap());
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add0() {
        assert_eq!(
            Solution::add(&String::from("999"), &String::from("99")),
            String::from("1098")
        )
    }

    #[test]
    fn multiply0() {
        assert_eq!(
            Solution::multiply(String::from("2"), String::from("3")),
            String::from("6")
        )
    }

    #[test]
    fn multiply1() {
        assert_eq!(
            Solution::multiply(String::from("12"), String::from("3")),
            String::from("36")
        )
    }

    #[test]
    fn multiply2() {
        assert_eq!(
            Solution::multiply(String::from("0"), String::from("123")),
            String::from("0")
        )
    }

    #[test]
    fn multiply3() {
        assert_eq!(
            Solution::multiply(String::from("123"), String::from("456")),
            String::from("56088")
        )
    }

    #[test]
    fn multiply4() {
        assert_eq!(
            Solution::multiply(String::from("99"), String::from("999")),
            String::from("98901")
        )
    }

    #[test]
    fn multiply5() {
        assert_eq!(
            Solution::multiply(String::from("9"), String::from("99")),
            String::from("891")
        )
    }

    #[test]
    fn multiply6() {
        assert_eq!(
            Solution::multiply(String::from("10"), String::from("999")),
            String::from("9990")
        )
    }

    #[test]
    fn multiply7() {
        assert_eq!(
            Solution::multiply(String::from("6"), String::from("501")),
            String::from("3006")
        )
    }
}
