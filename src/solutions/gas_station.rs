/// https://leetcode.com/problems/gas-station/description/

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // gas filled, needed and last valid position
    let (mut top_up, mut need, mut pos) = (0i32, 0i32, 0usize);
    for i in 0..gas.len() {
        // fill up the gas at every pos
        top_up += gas[i] - cost[i];
        if top_up.is_negative() {
            // means we need more gas in front
            need += top_up;
            top_up = 0; // zero the filled gas to reset state
            pos = (i + 1) % gas.len(); // set a new valid position
        }
    }
    // check if we need more than we can fill or otherwise
    if top_up + need >= 0 {
        pos as i32
    } else {
        -1
    }
}

// Copilot generated code for can_complete_circuit
pub fn _can_complete_circuit_copilot(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut total = 0;
    let mut tank = 0;

    for i in 0..gas.len() {
        tank += gas[i] - cost[i];
        if tank < 0 {
            start = i + 1;
            total += tank;
            tank = 0;
        }
    }

    if total + tank < 0 {
        return -1;
    }

    start as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn can_complete_circuit_works() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let result = super::can_complete_circuit(gas, cost);
        assert_eq!(result, 3);
    }
}
