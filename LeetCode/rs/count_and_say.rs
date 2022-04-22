struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            String::from("1")
        } else {
            let s = Self::count_and_say(n - 1);
            let mut count = 0;
            let mut ch = '0';
            let mut res = vec![];
            s.chars().for_each(|c| {
                if ch == '0' {
                    ch = c;
                    count = 1;
                } else {
                    if ch == c {
                        count += 1;
                    } else {
                        res.push(count.to_string());
                        res.push(ch.to_string());
                        ch = c;
                        count = 1;
                    }
                }
            });
            res.push(count.to_string());
            res.push(ch.to_string());

            res.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_and_say0() {
        assert_eq!(Solution::count_and_say(5), String::from("111221"));
    }
}
