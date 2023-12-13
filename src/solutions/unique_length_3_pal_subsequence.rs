/// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
use std::collections::{HashMap, HashSet};

fn fill_hashmap(s: &Vec<char>) -> HashMap<char, [Option<usize>; 2]> {
    let mut hashmap: HashMap<char, [Option<usize>; 2]> = HashMap::with_capacity(s.len());

    // move from left to right and populate first occurrences
    for (i, c) in s.iter().enumerate() {
        hashmap.entry(*c).or_insert([Some(i), None]);
    }

    // move from right to left and populate the last occurrences
    for (i, c) in s.iter().enumerate().rev() {
        hashmap.entry(*c).and_modify(|v| {
            if v[1].is_none() {
                v[1] = Some(i)
            }
        });
    }

    hashmap
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let hashmap: HashMap<char, [Option<usize>; 2]> = fill_hashmap(&s);
    let mut count = 0i32;
    for (_, pair) in hashmap {
        let (left, right) = (pair[0].unwrap(), pair[1].unwrap());
        let mut hashset: HashSet<char> = HashSet::with_capacity(right - left + 1);
        for i in left + 1..right {
            hashset.insert(s[i]);
        }
        count += hashset.len() as i32;
    }
    count
}

#[cfg(test)]
mod test {
    #[test]
    fn count_palindromic_subsequence_works() {
        let test_cases = vec![
            (String::from("aabca"), 3),
            (String::from("adc"), 0),
            (String::from("bbcbaba"), 4),
            (String::from("rfitbvukbx"), 3),
            (
                String::from("tlpjzdmtwderpkpmgoyrcxttiheassztncqvnfjeyxxp"),
                161,
            ),
        ];

        for test_case in test_cases {
            let actual = super::count_palindromic_subsequence(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}
