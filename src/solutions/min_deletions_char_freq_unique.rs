/// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/editorial/
use std::collections::{HashMap, HashSet};

pub fn min_deletions(s: String) -> i32 {
    let hashmap = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let mut freq = hashmap.into_iter().fold(Vec::new(), |mut v, n| {
        v.push(n.1);
        v
    });
    freq.sort_by(|&a, &b| b.cmp(&a));

    let mut hashset = HashSet::new();
    for i in 0..freq.len() {
        while hashset.contains(&freq[i]) && freq[i] > 0 {
            freq[i] -= 1;
        }
        hashset.insert(freq[i]);
    }

    (s.len() as i32) - freq.into_iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    #[test]
    fn min_deletions_works() {
        let test_cases = vec![("aab", 0), ("aaabbbcc", 2), ("ceabaacb", 2)];

        for test_case in test_cases {
            let got = super::min_deletions(test_case.0.to_string());
            assert_eq!(got, test_case.1);
        }
    }
}
