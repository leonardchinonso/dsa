use std::collections::HashSet;

/// https://leetcode.com/problems/sort-vowels-in-a-string/

pub fn sort_vowels(s: String) -> String {
    let vowels_set = HashSet::from(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u']);
    const ALPHABET_SIZE: usize = 123usize;
    let mut counts = vec![0i32; ALPHABET_SIZE];

    // replace all vowels with '#' and inc their count in the freq to enable counting sort
    let mut characters = s
        .chars()
        .filter_map(|c| {
            match vowels_set.contains(&c) {
                // check if the curr char is a vowel
                true => {
                    counts[c as usize] += 1; // inc the freq by 1
                    Some('#') // overwrite the position with a sentinel
                }
                false => Some(c), // return the char if it is a consonant
            }
        })
        .collect::<Vec<char>>();

    // read from the counts vector and write to the character vector
    let (mut write_pos, mut read_pos) = (0usize, 0usize);
    while write_pos < characters.len() {
        // look for the next valid character to read
        while read_pos < counts.len() && counts[read_pos] == 0 {
            read_pos += 1;
        }

        if read_pos == counts.len() {
            break;
        }

        // keep moving the position to write until we find a sentinel to replace
        while write_pos < characters.len() && characters[write_pos] != '#' {
            write_pos += 1;
        }

        if write_pos == characters.len() {
            break;
        }

        while counts[read_pos] > 0 && write_pos < characters.len() && characters[write_pos] == '#' {
            characters[write_pos] = (read_pos as u8) as char;
            write_pos += 1;
            counts[read_pos] -= 1;
        }
    }

    characters.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    #[test]
    fn sort_vowels_works() {
        let test_cases = vec![
            (String::from("lEetcOde"), String::from("lEOtcede")),
            (String::from("lYmpH"), String::from("lYmpH")),
            (String::from("sYFLdyXMkDBrbrzxbhRHHUZxwPVWGSMSFuPOCpUqJgboETGMdHSJIFuYWHuQdcjfNfvudOlVylHXtUz"), String::from("sYFLdyXMkDBrbrzxbhRHHEZxwPVWGSMSFIPOCpOqJgbUUTGMdHSJUFoYWHuQdcjfNfvudulVylHXtuz"))
        ];

        for test_case in test_cases {
            let actual = super::sort_vowels(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}
