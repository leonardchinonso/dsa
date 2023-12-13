pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod test {
    #[test]
    fn find_words_works() {
        let test_cases = vec![(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                String::from("oath"),
                String::from("pea"),
                String::from("eat"),
                String::from("rain"),
            ],
            vec![String::from("eat"), String::from("oath")],
        )];

        for test_case in test_cases {
            let got = super::find_words(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
