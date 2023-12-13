use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// https://leetcode.com/problems/invalid-transactions/
/// company: bloomberg

/**
idea:
- convert the Vec<transaction_str> to a Vec<Transaction>
- go through the Vec<Transaction> and push transactions with amount > 1000 to invalids
- group the Vec<Transaction> by name in a hashmap
- for each entry in the hashmap, check if the new transaction to insert is valid against it
**/

struct Transaction {
    name: String,
    time: i32,
    amount: i32,
    city: String,
}

impl From<String> for Transaction {
    fn from(value: String) -> Self {
        // assumption: the input is always correct
        let arr = value.split(",").collect::<Vec<&str>>();
        Self {
            name: arr[0].to_string(),
            time: i32::from_str(arr[1]).unwrap(),
            amount: i32::from_str(arr[2]).unwrap(),
            city: arr[3].to_string(),
        }
    }
}

impl Transaction {
    fn has_valid_amount(&self) -> bool {
        self.amount <= 1000
    }

    fn is_valid_comparison(&self, other: &Transaction) -> bool {
        if self.name != other.name || self.city == other.city || (self.time - other.time).abs() > 60
        {
            return true;
        }
        false
    }
}

pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    let mut invalids: HashSet<usize> = HashSet::with_capacity(transactions.len());
    let mut txs: HashMap<String, Vec<usize>> = HashMap::with_capacity(transactions.len());

    // convert all the transactions to Vec<Transaction>
    let transactions_object = transactions
        .iter()
        .map(|tx| Transaction::from(tx.clone()))
        .collect::<Vec<Transaction>>();

    for (id, transaction) in transactions_object.iter().enumerate() {
        if !transaction.has_valid_amount() {
            invalids.insert(id);
        }

        // if this user has not made any transactions,
        // insert the user, ensure the transaction is valid
        if txs.contains_key(&transaction.name) {
            let prev_tx_ids = txs.get(&transaction.name).unwrap();
            for prev_tx_id in prev_tx_ids {
                if !transaction.is_valid_comparison(&transactions_object[*prev_tx_id]) {
                    invalids.insert(id);
                    invalids.insert(*prev_tx_id);
                }
            }
        }

        // record this transaction for the user
        txs.entry(transaction.name.clone())
            .or_insert(vec![])
            .push(id);
    }

    invalids
        .into_iter()
        .map(|idx| transactions[idx].clone())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn invalid_transactions_works() {
        let test_cases = vec![
            (
                vec![
                    "alice,50,100,beijing".to_string(),
                    "alice,90,100,china".to_string(),
                ],
                vec![
                    "alice,50,100,beijing".to_string(),
                    "alice,90,100,china".to_string(),
                ],
            ),
            (
                vec![
                    "alice,20,800,mtv".to_string(),
                    "alice,50,100,beijing".to_string(),
                ],
                vec![
                    "alice,20,800,mtv".to_string(),
                    "alice,50,100,beijing".to_string(),
                ],
            ),
            (
                vec![
                    "alice,20,800,mtv".to_string(),
                    "alice,50,1200,mtv".to_string(),
                ],
                vec!["alice,50,1200,mtv".to_string()],
            ),
            (
                vec![
                    "alice,20,800,mtv".to_string(),
                    "bob,50,1200,mtv".to_string(),
                ],
                vec!["bob,50,1200,mtv".to_string()],
            ),
            (
                vec![
                    "alice,20,800,mtv".to_string(),
                    "alice,50,100,mtv".to_string(),
                    "alice,51,100,frankfurt".to_string(),
                ],
                vec![
                    "alice,20,800,mtv".to_string(),
                    "alice,50,100,mtv".to_string(),
                    "alice,51,100,frankfurt".to_string(),
                ],
            ),
        ];

        for test_case in test_cases {
            let got = super::invalid_transactions(test_case.0);
            let mut hashset_test_case =
                test_case.1.iter().fold(HashSet::new(), |mut hashset, s| {
                    hashset.insert(s.clone());
                    hashset
                });
            let mut hashset_got = got.iter().fold(HashSet::new(), |mut hashset, s| {
                hashset.insert(s.clone());
                hashset
            });

            for t in got {
                hashset_test_case.remove(t.as_str());
            }
            assert_eq!(hashset_test_case.len(), 0);

            for t in test_case.1 {
                hashset_got.remove(t.as_str());
            }
            assert_eq!(hashset_got.len(), 0);
        }
    }
}
