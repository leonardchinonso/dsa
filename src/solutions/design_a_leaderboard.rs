// use std::collections::{BinaryHeap, HashMap};
// use std::cmp::Reverse;
//
// struct Leaderboard {
//     scores: HashMap<i32, i32>
// }
//
// impl Leaderboard {
//
//     fn new() -> Self {
//         Self {
//             scores: HashMap::new()
//         }
//     }
//
//     fn add_score(&mut self, player_id: i32, score: i32) {
//         // let entry = self.scores.get_mut(&player_id);
//         // match entry {
//         //     Some(val) => *val += score,
//         //     None => self.scores.insert(player_id, score)
//         // }
//         self.scores.entry(player_id).and_modify(|s| *s += score).or_insert(score);
//     }
//
//     fn top(&self, k: i32) -> i32 {
//         let mut heap = BinaryHeap::with_capacity(k as usize);
//         for (_, score) in self.scores.iter() {
//             if heap.len() < k as usize {
//                 heap.push(Reverse(score));
//                 continue;
//             }
//
//             let lowest_score = heap.pop().unwrap();
//             if lowest_score.0 >= score {
//                 heap.push(lowest_score);
//                 continue;
//             }
//
//             heap.push(Reverse(score));
//         }
//
//         let mut sum = 0;
//         while let Some(v) = heap.pop() {
//             sum += v.0;
//         }
//         sum
//     }
//
//     fn reset(&mut self, player_id: i32) {
//         self.scores.remove(&player_id);
//     }
// }
