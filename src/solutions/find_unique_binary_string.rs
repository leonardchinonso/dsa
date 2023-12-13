/// https://leetcode.com/problems/find-unique-binary-string/

mod sub_optimal {
    use std::collections::HashSet;

    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let hashset: HashSet<String> = nums.into_iter().fold(HashSet::new(), |mut h, s| {
            h.insert(s);
            h
        });

        let curr_string = String::from("");
        find_binary_string(curr_string, &hashset, n).unwrap()
    }

    fn find_binary_string(
        mut curr_string: String,
        lookup: &HashSet<String>,
        n: usize,
    ) -> Option<String> {
        if curr_string.len() == n {
            if lookup.contains(&curr_string) {
                return None;
            }
            return Some(curr_string.clone());
        }

        curr_string.push('0');
        let added_zero_result = find_binary_string(curr_string.clone(), lookup, n);
        curr_string.pop();
        curr_string.push('1');
        return match added_zero_result {
            Some(result) => Some(result),
            None => find_binary_string(curr_string, lookup, n),
        };
    }
}

mod optimal {
    use std::collections::HashSet;

    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let hashset: HashSet<i64> = nums.into_iter().fold(HashSet::new(), |mut h, s| {
            h.insert(i64::from_str_radix(s.as_str(), 2).unwrap());
            h
        });

        for num in 0..n as i64 + 1 {
            if !hashset.contains(&num) {
                return pad_zeroes(String::from(format!("{num:b}")), n);
            }
        }

        String::new()
    }

    fn pad_zeroes(s: String, n: usize) -> String {
        let mut diff = n - s.len();
        let mut ret = String::new();
        while diff > 0 {
            ret.push('0');
            diff -= 1;
        }
        ret.push_str(s.as_str());
        ret
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn find_different_binary_string_works() {
        let test_cases = vec![
            vec![String::from("01"), String::from("10")],
            vec![String::from("00"), String::from("01")],
            vec![
                String::from("111"),
                String::from("011"),
                String::from("111"),
            ],
        ];

        for test_case in test_cases {
            let hashset = test_case.iter().fold(HashSet::new(), |mut h, s| {
                h.insert(s.clone());
                h
            });

            let actual = super::sub_optimal::find_different_binary_string(test_case.clone());
            assert!(!hashset.contains(&actual));

            let actual = super::optimal::find_different_binary_string(test_case);
            assert!(!hashset.contains(&actual));
        }
    }
}
