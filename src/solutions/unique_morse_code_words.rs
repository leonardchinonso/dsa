/// https://leetcode.com/problems/unique-morse-code-words
use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let codes = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let mut seen = HashSet::with_capacity(codes.len());
    for word in words {
        let mut code = String::new();
        for ch in word.chars() {
            let rep = codes[ch as usize - 'a' as usize];
            code.push_str(rep);
        }
        seen.insert(code);
    }

    seen.len() as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn unique_morse_representations_works() {
        let test_cases = vec![
            (
                vec![
                    "gin".to_string(),
                    "zen".to_string(),
                    "gig".to_string(),
                    "msg".to_string(),
                ],
                2,
            ),
            (vec!["a".to_string()], 1),
        ];

        for test_case in test_cases {
            let got = super::unique_morse_representations(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
