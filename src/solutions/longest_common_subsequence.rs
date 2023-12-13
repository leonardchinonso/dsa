/// https://leetcode.com/problems/longest-common-subsequence/

/*

t1 = abc
t2 = de

[-1, -1, -1]
[-1, -1, -1]
[-1, -1, -1]
[-1, -1, -1]

*/

mod top_down {
    use std::cmp::max;

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());

        // create a 2D vector to hold memory of visited subsequences
        let mut memo = vec![vec![-1; text2.len() + 1]; text1.len() + 1];
        let (m, n) = (memo.len(), memo[0].len());

        // make the last row and last column zeroes to use them as base cases
        memo[m - 1] = vec![0; text2.len() + 1];
        for i in 0..m - 1 {
            memo[i][n - 1] = 0;
        }

        recurse(0, 0, text1, text2, &mut memo)
    }

    fn recurse(p1: usize, p2: usize, s1: &[u8], s2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
        // if we have computed this string combination before, return it from the memo
        if memo[p1][p2] != -1 {
            return memo[p1][p2];
        }

        // compute the case where the first character in s1 IS NOT part of the optimal solution
        let res1 = recurse(p1 + 1, p2, s1, s2, memo);

        // compute the case where the first character in s1 IS part of the optimal solution
        // look for the first occurrence of s1's first character in s2 and start the search from there
        let res2 = match search_char_at(s2, s1[p1], p2) {
            Some(first_idx) => 1 + recurse(p1 + 1, first_idx + 1, s1, s2, memo),
            None => 0,
        };

        memo[p1][p2] = max(res1, res2);

        memo[p1][p2]
    }

    fn search_char_at(s: &[u8], key: u8, pos: usize) -> Option<usize> {
        for i in pos..s.len() {
            if s[i] == key {
                return Some(i);
            }
        }
        None
    }
}

mod top_down_optimal {
    use std::cmp::max;

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (mut text1, mut text2) = (text1.as_bytes(), text2.as_bytes());

        // create a 2D vector to hold memory of visited subsequences
        let mut memo = vec![vec![-1; text2.len() + 1]; text1.len() + 1];
        let (m, n) = (memo.len(), memo[0].len());

        // make the last row and last column zeroes to use them as base cases
        memo[m - 1] = vec![0; text2.len() + 1];
        for i in 0..m - 1 {
            memo[i][n - 1] = 0;
        }

        recurse(0, 0, text1, text2, &mut memo)
    }

    fn recurse(p1: usize, p2: usize, s1: &[u8], s2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
        // if we have computed this string combination before, return it from the memo
        if memo[p1][p2] != -1 {
            return memo[p1][p2];
        }

        let mut res = 0;
        // if the first characters of both strings are the same, add 1 and recurse
        // else move each pointer inward and check for possible long subsequences
        match s1[p1] == s2[p2] {
            true => {
                res = 1 + recurse(p1 + 1, p2 + 1, s1, s2, memo);
            }
            false => {
                let res1 = recurse(p1 + 1, p2, s1, s2, memo);
                let res2 = recurse(p1, p2 + 1, s1, s2, memo);
                res = max(res1, res2);
            }
        }

        memo[p1][p2] = res;
        res
    }
}

mod bottom_up {
    use std::cmp::max;

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (text1.len(), text2.len());

        // create a 2D vector to hold memory of visited subsequences
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        // starting from (m-1, n-1), build the solution upwards
        for mut j in (1..=n).rev() {
            j = j - 1;
            for mut i in (1..=m).rev() {
                i = i - 1;
                match text1[i] == text2[j] {
                    true => {
                        dp[i][j] = 1 + dp[i + 1][j + 1];
                    }
                    false => dp[i][j] = max(dp[i][j + 1], dp[i + 1][j]),
                }
            }
        }

        dp[0][0]
    }
}

mod bottom_up_optimal {
    use std::cmp::max;

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (mut text1, mut text2) = (text1.as_bytes(), text2.as_bytes());
        if text2.len() < text1.len() {
            (text1, text2) = (text2, text1);
        }

        let (m, n) = (text1.len(), text2.len());

        let mut prev_dp = vec![0; text1.len() + 1];

        // starting from (m-1, n-1) until 0, build the solution upwards
        for mut j in (1..=n).rev() {
            j = j - 1;
            // new vector to hold the current level
            let mut curr_dp = vec![0; text1.len() + 1];
            for mut i in (1..=m).rev() {
                i = i - 1;
                match text1[i] == text2[j] {
                    true => {
                        curr_dp[i] = 1 + prev_dp[i + 1];
                    }
                    false => curr_dp[i] = max(prev_dp[i], curr_dp[i + 1]),
                }
            }
            prev_dp = curr_dp;
        }

        prev_dp[0]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn longest_common_subsequence_works() {
        let test_cases = vec![
            ("abcde".to_string(), "ace".to_string(), 3),
            ("abc".to_string(), "abc".to_string(), 3),
            ("abc".to_string(), "def".to_string(), 0),
            ("".to_string(), "".to_string(), 0),
            ("abcde".to_string(), "acer".to_string(), 3),
        ];

        for test_case in test_cases {
            let got = super::top_down::longest_common_subsequence(
                test_case.0.clone(),
                test_case.1.clone(),
            );
            assert_eq!(got, test_case.2);
            let got = super::top_down_optimal::longest_common_subsequence(
                test_case.0.clone(),
                test_case.1.clone(),
            );
            assert_eq!(got, test_case.2);
            let got = super::bottom_up::longest_common_subsequence(
                test_case.0.clone(),
                test_case.1.clone(),
            );
            assert_eq!(got, test_case.2);
            let got =
                super::bottom_up_optimal::longest_common_subsequence(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
