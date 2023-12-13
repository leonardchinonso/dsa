use std::collections::HashMap;

pub fn unique_letter_string(s: String) -> i32 {
    let mut hashmap: HashMap<char, Vec<i32>> = HashMap::new();
    let size = s.len();

    for ch in s.chars() {
        let mut v = Vec::with_capacity(size);
        v.push(-1);
        hashmap.insert(ch, v); // overwrite if ch already in hashmap
    }

    for (idx, ch) in s.chars().enumerate() {
        let vector = hashmap.get_mut(&ch).unwrap();
        vector.push(idx as i32);
    }

    for ch in s.chars() {
        let vector = hashmap.get_mut(&ch).unwrap();
        vector.push(size as i32);
    }

    collate(&hashmap)
}

fn collate(hashmap: &HashMap<char, Vec<i32>>) -> i32 {
    let mut runner = 0_i32;
    for (_, vector) in hashmap {
        for j in 1..vector.len() - 1 {
            runner += (vector[j] - vector[j - 1]) * (vector[j + 1] - vector[j]);
        }
    }
    runner
}

mod test {
    #[test]
    fn unique_letter_string_works() {
        let test_cases = vec![
            ("ABC".to_string(), 10),
            ("ABA".to_string(), 8),
            ("LEETCODE".to_string(), 92),
        ];

        for test_case in test_cases {
            let got = super::unique_letter_string(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
