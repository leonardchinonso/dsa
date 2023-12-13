pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let pivot = find_pivot(&nums).unwrap();

    if target == nums[pivot] {
        return pivot as i32;
    }

    if target > *nums.last().unwrap() {
        return binary_search(0, pivot, nums, target);
    }

    binary_search(pivot, nums.len() - 1, nums, target)
}

fn find_pivot(nums: &Vec<i32>) -> Option<usize> {
    let (mut start, mut end) = (0, nums.len() - 1);
    while start <= end {
        let mid = (start + end) / 2;

        // if mid is on the boundary
        if mid == start || mid == end {
            if nums[end] < nums[start] {
                return Some(end);
            }
            return Some(start);
        }

        // if mid is the pivot
        if nums[mid] < nums[mid - 1] && nums[mid] < nums[mid + 1] {
            return Some(mid);
        }

        // if pivot is in the right half
        if nums[mid] > nums[end] {
            start = mid + 1;
        } else {
            // if pivot is in the right half
            end = mid - 1;
        }
    }

    // ideally the code should never get here
    None
}

fn binary_search(mut start: usize, mut end: usize, nums: Vec<i32>, target: i32) -> i32 {
    while start <= end {
        let mid = (start + end) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] < target {
            start = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            end = mid - 1;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    #[test]
    fn search_works() {
        let test_cases = vec![
            (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
            (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
            (vec![1], 0, -1),
            (vec![3, 1], 3, 0),
            (vec![8, 9, 2, 3, 4], 9, 1),
        ];

        for test_case in test_cases {
            let actual = super::search(test_case.0, test_case.1);
            assert_eq!(actual, test_case.2);
        }
    }
}
