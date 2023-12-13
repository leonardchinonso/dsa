/// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description
use std::collections::{HashMap, HashSet};

pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let s1_count = freq_count(&s1);
    let s2_count = freq_count(&s2);
    let lcs_map = lcs(s1, s2);

    let s1_ascii_count = compute_rem_ascii(s1_count, &lcs_map);
    let s2_ascii_count = compute_rem_ascii(s2_count, &lcs_map);

    s1_ascii_count + s2_ascii_count
}

/// compute_rem_ascii computes the remaining ascii values after removing the sum of the lcs ascii
fn compute_rem_ascii(
    mut s_freq_count: HashMap<char, usize>,
    lcs_map: &HashMap<char, usize>,
) -> i32 {
    let mut ascii_count = 0_u32;
    for (ch, freq) in s_freq_count.iter_mut() {
        if let Some(val) = lcs_map.get(ch) {
            *freq -= val
        }
        ascii_count += *ch as u32 * *freq as u32;
    }
    ascii_count as i32
}

/// freq_count derives a character frequency map from a string
fn freq_count(s: &String) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut hashmap, ch| {
        *hashmap.entry(ch).or_insert(0) += 1;
        hashmap
    })
}

/// lcs finds the LCS of two strings using lcs_dfs
/// returns the freq hashmap of the resulting LCS
fn lcs(s1: String, s2: String) -> HashMap<char, usize> {
    let mut dp: HashMap<(&[char], &[char]), String> = HashMap::new();
    let mut seen: HashSet<(&[char], &[char])> = HashSet::new();
    let v1 = s1.chars().collect::<Vec<char>>();
    let v2 = s2.chars().collect::<Vec<char>>();

    let s = lcs_dfs(&v1[..], &v2[..], &mut seen, &mut dp)
        .chars()
        .rev()
        .collect::<String>();
    let hashmap = freq_count(&s);
    hashmap
}

/// lcs_dfs uses dfs to find the LCS of two strings represented as a vector of characters
fn lcs_dfs<'a>(
    v1: &'a [char],
    v2: &'a [char],
    seen: &mut HashSet<(&'a [char], &'a [char])>,
    dp: &mut HashMap<(&'a [char], &'a [char]), String>,
) -> String {
    if v1.is_empty() || v2.is_empty() {
        return "".to_string();
    }

    // create a pair to identify the combination
    let dp_index = (v1, v2);

    // if we've seen this pair before, return the stored answer
    if seen.contains(&dp_index) {
        return dp.get(&dp_index).unwrap().clone();
    }
    // else mark it as seen
    seen.insert((v1, v2));

    // if the last two characters match, check lcs of the remaining n-1 characters
    if v1.last().unwrap() == v2.last().unwrap() {
        let lcs_rem = lcs_dfs(&v1[..v1.len() - 1], &v2[..v2.len() - 1], seen, dp);
        let res = format!("{}{}", v1.last().unwrap(), lcs_rem);
        dp.insert((v1, v2), res.clone());
        return res;
    }

    // if last two characters don't match, remove last character of each string and compare the length
    let res = max_str(
        lcs_dfs(&v1[..v1.len() - 1], v2, seen, dp),
        lcs_dfs(v1, &v2[..v2.len() - 1], seen, dp),
    );
    dp.insert((v1, v2), res.clone());
    return res;
}

/// max_str returns the string with the higher ascii sum of two strings
fn max_str(s1: String, s2: String) -> String {
    let s1_sum = ascii_sum(s1.clone());
    let s2_sum = ascii_sum(s2.clone());
    if s1_sum > s2_sum {
        return s1;
    }
    s2
}

/// ascii_sum sums chars in a string by their ascii values
fn ascii_sum(s: String) -> u32 {
    s.chars().fold(0, |mut sum, ch| {
        sum += ch as u32;
        sum
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn minimum_delete_sum_works() {
        let test_cases = vec![("sea", "eat", 231), ("delete", "leet", 403)];

        for test_case in test_cases {
            let got = super::minimum_delete_sum(test_case.0.to_string(), test_case.1.to_string());
            assert_eq!(got, test_case.2);
        }
    }
}
