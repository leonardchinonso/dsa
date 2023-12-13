use std::cmp::min;

/// brute_force groups the code for the brute force solution
mod brute_force {
    use super::*;

    // permute gets all possible rearrangements of characters in a string and stores it in result
    fn permute(char_stack: &mut Vec<char>, remaining: &mut Vec<char>, result: &mut Vec<String>) {
        // if there is nothing more to process in the remaining
        // take all items from the character stack and put it in the result
        if remaining.is_empty() {
            result.push(char_stack.iter().collect::<String>());
            return;
        }

        // if it gets here, there are still elements to process in remaining
        for pos in 0..remaining.len() {
            // remove a letter from the remaining string at the current position
            // remaining will shift other letters to the left
            let char_to_insert = remaining.remove(pos);
            // push it onto the character stack
            char_stack.push(char_to_insert);

            // recursively call the permute function on the new char_stack
            permute(char_stack, remaining, result);

            // this is the backtracking part,
            // pop the character from the character stack to return it to its former state
            let popped_character = char_stack.pop().unwrap();

            // insert the letter again into the remaining string at the same position
            remaining.insert(pos, popped_character);
        }
    }

    // get_permutations gets all possible permutations of a string
    fn get_permutations(num: String) -> Vec<String> {
        let mut result = Vec::new();

        // create a stack to hold intermediary permutations
        let mut char_stack: Vec<char> = Vec::new();

        // create a vector to hold the remaining unvisited characters
        // it will be all of the characters initially
        let mut remaining: Vec<char> = num.chars().collect();

        // permute the string num and store it in result
        permute(&mut char_stack, &mut remaining, &mut result);

        result
    }

    // next_greater_element_brute_force uses the brute force approach of getting all possible
    // permutations of a number and picking the one JUST greater than the number
    pub fn next_greater_element(n: i32) -> i32 {
        // get all permutations of n
        let strings = get_permutations(n.to_string());

        // store the largest possible number in minimum
        let mut minimum: i64 = (i32::MAX as i64) + 1;
        for s in strings {
            // if the number can fit in the 32-bit space and it is greater than n,
            // compare it to the minimum value so far and write if its greater
            if let Ok(n_32) = s.parse::<i32>() {
                if n_32 > n {
                    minimum = min(minimum, n_32 as i64)
                }
            }
        }

        // if the minimum did not change, return -1, no smaller numbers found
        minimum = if minimum < (i32::MAX as i64) + 1 {
            minimum
        } else {
            -1
        };
        minimum as i32
    }
}

/// optimal groups the code for the more optimal solution
mod optimal {
    // next_greater_element_optimal finds the first digit that JUST breaks the invariant
    // where the smaller digit could be less and tries to reverse it
    pub fn next_greater_element(n: i32) -> i32 {
        let mut n_vec = to_int_vec(n);
        // if there is only one digit in the string, return -1
        if n_vec.len() == 1 {
            return -1;
        }

        // pivot is the index at which the split and reverse happens
        let mut pivot: i32 = -1;
        // from the RHS, look for the first number that's smaller than the one after it
        for idx in (0..=n_vec.len() - 2).rev() {
            if n_vec[idx] < n_vec[idx + 1] {
                pivot = idx as i32;
                break;
            }
        }
        // if no pivot is found, return -1, we can't get a smaller number
        if pivot == -1 {
            return -1;
        }

        // look for the first number that's greater than the pivot and swap it with pivot
        for idx in (0..=n_vec.len() - 1).rev() {
            if n_vec[idx] > n_vec[pivot as usize] {
                n_vec.swap(idx, pivot as usize);
                break;
            }
        }

        // get the start and end points to reverse the vector
        let (start, end) = ((pivot + 1) as usize, n_vec.len() - 1);
        // reverse the pivot end of the list
        reverse(&mut n_vec, start, end);

        // if it overflows, return -1
        match to_int_by_str(n_vec) {
            Some(n_32) => n_32,
            None => -1,
        }
    }

    // to _int_vec converts an integer to a vector of digits of that integer
    fn to_int_vec(mut n: i32) -> Vec<i32> {
        let mut digits = Vec::new();
        while n > 0 {
            // pop the last digit from n
            digits.push(n % 10);
            n = n / 10;
        }
        // reverse the vector to get the order back
        digits.reverse();
        digits
    }

    // reverse reverses the elements between start and end in place
    fn reverse(vector: &mut Vec<i32>, mut start: usize, mut end: usize) {
        while start < end {
            vector.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    // to_int converts a vector of digits to a single integer
    // returns a None if it overflows
    fn _to_int(vector: Vec<i32>) -> Option<i32> {
        let mut result: i64 = 0;
        // if the len of vec is 3, n should be 10^2
        let mut n = 10_i32.pow((vector.len() - 1) as u32);

        for val in vector {
            result += (val * n) as i64;
            n /= 10; // reduce the bias
        }

        // if result overflows the 32-bit integer, return None
        if result > i32::MAX as i64 {
            return None;
        }

        Some(result as i32)
    }

    // to_int converts a vector of digits to a single integer by converting to string first
    // returns a None if it overflows
    fn to_int_by_str(vector: Vec<i32>) -> Option<i32> {
        let mut n_str = String::new();
        // convert the integer vector to a string
        for val in vector {
            n_str.push_str(val.to_string().as_str())
        }
        // if the parsed string can fit in a 32bit integer, return it
        // else return None
        match n_str.parse::<i32>() {
            Ok(n_32) => Some(n_32),
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_CASES: [(i32, i32); 5] = [
        (12, 21),
        (123, 132),
        (21, -1),
        (2147483476, 2147483647),
        (1999999999, -1),
    ];

    #[test]
    fn next_greater_element_brute_force_works() {
        for (input, expected) in TEST_CASES {
            let got = brute_force::next_greater_element(input);
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn next_greater_element_optimal_works() {
        for (input, expected) in TEST_CASES {
            let got = optimal::next_greater_element(input);
            assert_eq!(got, expected);
        }
    }
}
