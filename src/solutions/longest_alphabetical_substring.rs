use std::cmp::max;

/// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/

pub fn longest_continuous_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len() as i32;
    let mut longest = 0;
    for i in 0..s.len() {
        longest = max(longest, expand_from_middle(s, i as i32, n));
        if longest == n {
            return longest;
        }
    }
    longest
}

fn expand_from_middle(bytes: &[u8], idx: i32, n: i32) -> i32 {
    let (mut left, mut right) = (idx, idx);

    while left > 0 && right <= n - 1 {
        if left - 1 >= 0 {
            if is_one_greater(bytes[left as usize], bytes[left as usize - 1]) {
                left -= 1;
            } else {
                break;
            }
        }

        if right + 1 < n {
            if is_one_greater(bytes[right as usize + 1], bytes[right as usize]) {
                right += 1;
            } else {
                break;
            }
        }
    }

    while left - 1 >= 0 && is_one_greater(bytes[left as usize], bytes[left as usize - 1]) {
        left -= 1;
    }

    while right + 1 < n && is_one_greater(bytes[right as usize + 1], bytes[right as usize]) {
        right += 1;
    }

    right - left + 1
}

fn is_one_greater(big: u8, small: u8) -> bool {
    if big > small && big - small == 1 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn longest_continuous_substring_works() {
        let test_cases = vec![
            ("abacaba".to_string(), 2),
            ("abcde".to_string(), 5),
            ("awy".to_string(), 1),
        ];

        for test_case in test_cases {
            let got = super::longest_continuous_substring(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
