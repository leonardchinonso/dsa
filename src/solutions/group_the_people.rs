/// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
use std::collections::HashMap;
pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map_ptr = 0_i32;
    let hashmap: HashMap<usize, Vec<i32>> =
        group_sizes
            .iter()
            .fold(HashMap::new(), |mut hashmap, size| {
                hashmap
                    .entry(*size as usize)
                    .or_insert(Vec::new())
                    .push(map_ptr);
                map_ptr += 1;
                hashmap
            });

    let mut result: Vec<Vec<i32>> = Vec::with_capacity(group_sizes.len());

    for (freq, mut values) in hashmap {
        let mut vector: Vec<i32> = Vec::with_capacity(freq);
        while !values.is_empty() {
            vector.push(values.pop().unwrap());
            if vector.len() == freq {
                result.push(vector);
                vector = Vec::with_capacity(freq);
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    fn compare_vectors_for_equal_digits(
        mut vector_one: Vec<Vec<i32>>,
        mut vector_two: Vec<Vec<i32>>,
    ) {
        for v in vector_one.iter_mut() {
            v.sort();
        }
        for v in vector_two.iter_mut() {
            v.sort();
        }

        vector_one.sort_by(|a, b| a.len().cmp(&b.len()));
        vector_two.sort_by(|a, b| a.len().cmp(&b.len()));

        assert_eq!(vector_one, vector_two);
    }

    #[test]
    fn group_the_people_works() {
        let test_cases = vec![
            (
                vec![3, 3, 3, 3, 3, 1, 3],
                vec![vec![5], vec![3, 4, 6], vec![0, 1, 2]],
            ),
            (
                vec![2, 1, 3, 3, 3, 2],
                vec![vec![1], vec![0, 5], vec![2, 3, 4]],
            ),
        ];

        for test_case in test_cases {
            let got = super::group_the_people(test_case.0);
            compare_vectors_for_equal_digits(got, test_case.1)
        }
    }
}
