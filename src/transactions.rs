use serde::{Deserialize};
use csv::{ReaderBuilder, Trim};
use std::io::Read;


#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub struct Transaction {
    #[serde(rename="type")]
    tx_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f32,
}
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.tx_type == other.tx_type &&
        self.client == other.client &&
        self.tx == other.tx &&
        self.amount == other.amount
    }

}

pub fn for_each_transaction_in<F>(source: impl Read, mut do_for_each: F) 
    where F: FnMut(Transaction) {
     let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .trim(Trim::All)
        .from_reader(source);
     for record in rdr.deserialize() {
         do_for_each(record.unwrap());
     }
} 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_csv() {
        let csv = "\
        type, client, tx, amount
        deposit,1, 1, 1.0";
        let source = std::io::Cursor::new(csv.as_bytes());
        let expected = Transaction {
            tx_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: 1.0,
        };
        let mut obtained: Vec<Transaction> = Vec::<Transaction>::new();
        for_each_transaction_in(source, move |item: Transaction| {
            assert_eq!(expected, item);
            println!("got {:?}", item);
            obtained.push(item)
        });
    }

    #[test]
    fn test_load_csv_withdrawal() {
        let csv = "\
        type, client, tx, amount
        withdrawal, 2, 5, 3.0";
        let source = std::io::Cursor::new(csv.as_bytes());
        let expected = Transaction {
            tx_type: TransactionType::Withdrawal,
            client: 2,
            tx: 5,
            amount: 3.0,
        };
        let mut obtained: Vec<Transaction> = Vec::<Transaction>::new();
        for_each_transaction_in(source, move |item: Transaction| {
            println!("got {:?}", item);
            assert_eq!(item, expected);
            obtained.push(item)
        });
    }
}
