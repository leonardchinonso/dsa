use std::cmp::min;
use std::collections::HashMap;

pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    let mut min_dist = i32::MAX;
    for (idx, card) in cards.into_iter().enumerate() {
        match hashmap.get_mut(&card) {
            Some(last_idx) => {
                min_dist = min(min_dist, (idx - *last_idx) as i32);
                *last_idx = idx;
            }
            None => {
                hashmap.insert(card, idx);
            }
        }
    }
    min_dist = if min_dist == i32::MAX {
        -1
    } else {
        min_dist + 1
    };
    min_dist
}

#[cfg(test)]
mod test {
    #[test]
    fn minimum_card_pickup_works() {
        let test_cases = vec![(vec![3, 4, 2, 3, 4, 7], 4), (vec![1, 0, 5, 3], -1)];

        for test_case in test_cases {
            let got = super::minimum_card_pickup(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
