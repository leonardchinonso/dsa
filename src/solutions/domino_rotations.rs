/// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/

/*
Idea
    tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
    Top
    2 - 1, 3
    Bottom
    2 - 0, 2, 4

    tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
    Top
    3 - 1, 2, 3
    Bottom
    3 - 1, 4
*/

mod brute_force {
    use std::cmp::min;
    use std::collections::{HashMap, HashSet};

    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        if all_same(&tops) || all_same(&bottoms) {
            return 0;
        }

        let mut rotations = i32::MAX;
        let n = tops.len();

        let tops_map = create_domino_freq_map(tops);
        let bottoms_map = create_domino_freq_map(bottoms);

        for (domino, tops_set) in tops_map {
            let mut bottoms_set_len = 0;
            let tops_set_len = tops_set.len();
            let total_len = match bottoms_map.get(&domino) {
                None => 0_usize,
                Some(bottoms_set) => {
                    bottoms_set_len = bottoms_set.len();
                    bottoms_set_len + tops_set_len
                }
            };

            dbg!(domino);
            dbg!(total_len);
            dbg!(n);
            // println!("Domino: {domino}, total_len: {}");

            if total_len == n {
                println!("GOt here");
                rotations = min(rotations, min(tops_set_len, bottoms_set_len) as i32)
            }
        }

        if rotations == i32::MAX {
            -1
        } else {
            rotations
        }
    }

    fn all_same(slice: &[i32]) -> bool {
        if slice.is_empty() {
            return false;
        }
        for i in 0..slice.len() {
            if slice[i] != slice[0] {
                return false;
            }
        }
        true
    }

    fn create_domino_freq_map(domino_deck: Vec<i32>) -> HashMap<i32, HashSet<usize>> {
        domino_deck
            .iter()
            .fold(HashMap::new(), |mut hashmap, domino| {
                for (idx, other) in domino_deck.iter().enumerate() {
                    if domino == other {
                        continue;
                    }
                    hashmap.entry(*domino).or_insert(HashSet::new()).insert(idx);
                }
                hashmap
            })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn min_domino_rotations_works() {
        let test_cases = vec![
            (vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2], 2),
            // (vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4], -1),
        ];

        for test_case in test_cases {
            let got = super::brute_force::min_domino_rotations(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
