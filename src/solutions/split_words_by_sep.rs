pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    let mut final_result: Vec<String> = Vec::new();

    for word in words {
        final_result.extend(split_by(word, separator))
    }

    final_result
}

// split_by splits a string by a given separator
pub fn split_by(string: String, sep: char) -> Vec<String> {
    if string.is_empty() {
        return vec![];
    }

    let mut result: Vec<String> = Vec::new();
    let mut curr = String::new();
    for ch in string.chars() {
        if sep == ch {
            if !curr.is_empty() {
                result.push(curr.clone());
            }
            curr = String::new();
            continue;
        }

        curr.push(ch)
    }

    if !curr.is_empty() {
        result.push(curr);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_words_by_separator_works() {
        struct TestCase {
            input_words: Vec<String>,
            input_sep: char,
            expected: Vec<String>,
        }

        impl TestCase {
            fn new(input_words: Vec<String>, input_sep: char, expected: Vec<String>) -> Self {
                Self {
                    input_words,
                    input_sep,
                    expected,
                }
            }
        }

        let test_cases: Vec<TestCase> = vec![TestCase::new(
            vec![
                String::from("one.two.three"),
                String::from("four.five"),
                String::from("six"),
            ],
            '.',
            vec![
                String::from("one"),
                String::from("two"),
                String::from("three"),
                String::from("four"),
                String::from("five"),
                String::from("six"),
            ],
        )];

        for test_case in test_cases {
            let got = split_words_by_separator(test_case.input_words, test_case.input_sep);
            assert_eq!(got, test_case.expected);
        }
    }
}
