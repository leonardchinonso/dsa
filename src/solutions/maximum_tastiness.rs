/// https://leetcode.com/problems/maximum-tastiness-of-candy-basket/

// can_place checks if count number of items can be placed dist apart in arr
fn can_place(arr: &Vec<i32>, dist: i32, mut count: i32) -> bool {
    let mut last = 0;
    for next in 1..arr.len() {
        // if count is one, then the check is complete
        if count == 1 {
            return true;
        }

        // if the distance between the next and last is enough
        // move last to next and reduce the count
        if arr[next] - arr[last] >= dist {
            last = next;
            count -= 1
        }
    }

    // if it finishes the process with count as 1, then all can be placed
    if count == 1 {
        return true;
    }

    false
}

/// linear_search mod contains the approach using linear search
mod linear_search {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        // sort the array in ascending order
        let mut sorted_price = price.clone();
        sorted_price.sort();

        let max_possible_dist = sorted_price[sorted_price.len() - 1] - sorted_price[0];
        for dist in 1..=max_possible_dist {
            // check if we can place the remaining k values
            if !super::can_place(&sorted_price, dist, k) {
                // if we cannot place items in the array at
                // dist apart, then return the last possible
                // placing distance
                return dist - 1;
            }
        }

        // we can place all of k with any distance since can_place always worked
        // return the maximum distance possible
        max_possible_dist
    }
}

/// binary_search mod contains the approach using linear search
mod binary_search {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        // sort the array in ascending order
        let mut sorted_price = price.clone();
        sorted_price.sort();

        let max_possible_dist = sorted_price[sorted_price.len() - 1] - sorted_price[0];

        let (mut start, mut end, mut last) = (1, max_possible_dist + 1, 0_i32);
        // using binary search, find the position to place the value
        while start <= end {
            let dist = (start + end) / 2; // get a possible distance
                                          // if this distance works for all k values, move start to look for a farther distance
            if super::can_place(&sorted_price, dist, k) {
                last = dist;
                start = dist + 1;
            } else {
                // if it does not work for all k values, move end to look for a closer distance
                end = dist - 1;
            }
        }

        last
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
        return vec![
            (vec![13, 5, 1, 8, 21, 2], 3, 8),
            (vec![1, 3, 1], 2, 2),
            (vec![7, 7, 7, 7], 2, 0),
            (vec![63, 85, 135, 16, 200, 168, 159, 28], 6, 22),
        ];
    }

    #[test]
    fn linear_search_works() {
        let test_cases = get_test_cases();

        for test_case in test_cases {
            let got = linear_search::maximum_tastiness(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }

    #[test]
    fn binary_search_works() {
        let test_cases = get_test_cases();

        for test_case in test_cases {
            let got = binary_search::maximum_tastiness(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
