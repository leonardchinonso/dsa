/// https://leetcode.com/problems/game-of-nim/
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Hash, Clone)]
struct State(Vec<i32>);

/// State represents a sorted piles vector
impl State {
    fn derive_child_states(&self) -> Vec<TransitionData> {
        let mut transition_vector: Vec<TransitionData> = Vec::new();
        for i in 0..self.0.len() {
            let mut next_state = self.0.clone();
            for j in 0..self.0[i] {
                next_state[i] = j;
                let next_state = State::from(next_state.clone());
                let amount = self.0[i] - j;
                let transition_data = TransitionData::new(next_state, amount);
                transition_vector.push(transition_data);
            }
        }
        transition_vector
    }
}

impl From<Vec<i32>> for State {
    fn from(mut value: Vec<i32>) -> Self {
        value.sort();
        Self(value)
    }
}

struct TransitionData {
    state: State,
    amount: i32,
}

impl TransitionData {
    fn new(state: State, amount: i32) -> Self {
        Self { state, amount }
    }
}

pub fn nim_game(piles: Vec<i32>) -> bool {
    let moves: i32 = piles.iter().sum();
    let mut memo: HashMap<State, bool> = HashMap::new();
    let curr_state = State::from(piles);
    player_move(curr_state, &mut memo, moves)
}

fn player_move(curr_state: State, memo: &mut HashMap<State, bool>, moves: i32) -> bool {
    // check if current player can make any moves
    if moves == 0 {
        return false;
    }

    // check if this state has been seen before
    if memo.contains_key(&curr_state) {
        return *memo.get(&curr_state).unwrap();
    }

    let child_states: Vec<TransitionData> = curr_state.derive_child_states();
    for child_state in child_states {
        // if the next turn can be bad, take advantage of it
        if !player_move(child_state.state, memo, moves - child_state.amount) {
            memo.insert(curr_state.clone(), true);
            return true;
        }
    }

    memo.insert(curr_state.clone(), false);
    false
}

#[cfg(test)]
mod test {
    #[test]
    fn nim_game_works() {
        let test_cases = vec![
            (vec![1], true),
            (vec![1, 1], false),
            (vec![1, 2, 3], false),
            (vec![7], true),
            (vec![2, 2], false),
        ];

        for test_case in test_cases {
            let actual = super::nim_game(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}
