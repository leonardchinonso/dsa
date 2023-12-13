/// https://leetcode.com/problems/decode-string/
/// company: bloomberg

pub fn decode_string(s: String) -> String {
    let mut string_stack = Vec::new();
    let mut digit_stack = Vec::new();
    let mut curr_string = String::new();
    let mut k = 0;

    for ch in s.chars() {
        // if it is a digit
        if ch.is_ascii_digit() {
            k = k * 10 + ch as u8 - '0' as u8;
            continue;
        }

        // if it is an opening bracket
        if ch == '[' {
            digit_stack.push(k);
            string_stack.push(curr_string.clone());

            k = 0;
            curr_string.clear();
            continue;
        }

        // if it is a closing bracket
        if ch == ']' {
            let mut decoded = string_stack.pop().unwrap();
            for _ in (0..digit_stack.pop().unwrap()).rev() {
                decoded.push_str(curr_string.clone().as_str());
            }
            curr_string = decoded;
            continue;
        }

        // must be a letter
        curr_string.push(ch);
    }

    curr_string
}

#[cfg(test)]
mod test {
    #[test]
    fn decode_string_works() {
        let test_cases = vec![
            (String::from("3[a]2[bc]"), String::from("aaabcbc")),
            (String::from("3[a2[c]]"), String::from("accaccacc")),
            (
                String::from("2[abc]3[cd]ef"),
                String::from("abcabccdcdcdef"),
            ),
            (
                String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef"),
                String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef"),
            ),
        ];

        for test_case in test_cases {
            let got = super::decode_string(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
