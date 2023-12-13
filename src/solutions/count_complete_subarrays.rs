/// brute_force contains the brute force solution to the problem
mod brute_force {
    use std::collections::HashSet;

    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        set.extend(nums.clone());

        if nums.len() == set.len() {
            return 1;
        }

        let mut count = 0;

        for i in 0..nums.len() {
            // create a new set to count the distinct elements in a subarray
            let mut inner_set = HashSet::new();
            for j in i..nums.len() {
                // insert the value in the inner set
                inner_set.insert(nums[j]);
                // if the inner set has all the values, then it is a complete subarray
                if inner_set.len() == set.len() {
                    count += 1;
                }
            }
        }

        count
    }
}

mod optimal {
    use std::collections::HashMap;

    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for num in nums.iter() {
            // hashmap.entry(*num).and_modify(|val| *val += 1).or_insert(1);
            match hashmap.get_mut(num) {
                Some(val) => *val += 1,
                None => {
                    hashmap.insert(*num, 1);
                }
            }
        }

        let (num_of_distinct, mut count) = (hashmap.len(), 0i32);
        hashmap.clear();

        let (mut i, mut j) = (0usize, 0usize);
        while i < nums.len() && j < nums.len() {
            hashmap
                .entry(nums[j])
                .and_modify(|val| *val += 1)
                .or_insert(1);
            while i < nums.len() && num_of_distinct == hashmap.len() {
                count += (nums.len() - j) as i32;
                *hashmap.get_mut(&nums[i]).unwrap() -= 1;
                if hashmap.get(&nums[i]).unwrap() == &0 {
                    hashmap.remove(&nums[i]);
                }
                i += 1;
            }
            j += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_complete_subarrays_brute_force_works() {
        let test_cases = vec![(vec![1, 3, 1, 2, 2], 4), (vec![5, 5, 5, 5], 10)];

        for test_case in test_cases {
            let got = brute_force::count_complete_subarrays(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }

    #[test]
    fn count_complete_subarrays_optimal_works() {
        let test_cases = vec![(vec![1, 3, 1, 2, 2], 4), (vec![5, 5, 5, 5], 10)];

        for test_case in test_cases {
            let got = optimal::count_complete_subarrays(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
