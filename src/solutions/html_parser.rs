use std::collections::HashMap;

/// https://leetcode.com/problems/html-entity-parser/

pub fn entity_parser(text: String) -> String {
    let dict = HashMap::from([
        ("&amp;", "&"),
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&frasl;", "/"),
    ]);

    let mut result = String::with_capacity(text.len());
    let mut ch_iter = text.chars();
    while let Some(ch) = ch_iter.next() {
        if ch == '&' {
            let mut s = String::from(ch);
            while let Some(ch) = ch_iter.next() {
                if ch == '&' {
                    result.push_str(s.as_str());
                    s.clear();
                    s.push(ch);
                    continue;
                }
                s.push(ch);
                if !ch.is_ascii_alphabetic() {
                    break;
                }
            }
            result.push_str({
                let res = dict.get(s.as_str());
                if res.is_none() {
                    s.as_str()
                } else {
                    res.unwrap()
                }
            });
            continue;
        }
        result.push(ch);
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn entity_parser_works() {
        let test_cases = vec![
            (
                String::from("&amp; is an HTML entity but &ambassador; is not."),
                String::from("& is an HTML entity but &ambassador; is not."),
            ),
            (
                String::from("and I quote: &quot;...&quot;"),
                String::from("and I quote: \"...\""),
            ),
            (String::from("&&gt;"), String::from("&>")),
            (
                String::from("&&gt;&gt;&&&lt&&&&lt;"),
                String::from("&>>&&&lt&&&<"),
            ),
        ];

        for test_case in test_cases {
            let got = super::entity_parser(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
