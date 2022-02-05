use std::error::Error;
use std::io;
use std::process;
use serde::{Deserialize};
extern crate csv;
use csv::{ReaderBuilder};

 
fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Deposit,
    Withdrawal
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub struct Record {
    #[serde(rename="type")]
    operation: OperationType,
    client: u16,
    tx: u32,
    amount: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let csv = "\
type, client, tx, amount
deposit,1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0";

     let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .trim(csv::Trim::All)
        .from_reader(csv.as_bytes());
     let mut iter = rdr.deserialize();

     // Read the first record.
     if let Some(result) = iter.next() {
         let record: Record = result?;
         assert_eq!(record, Record {
             operation: OperationType::Deposit,
             client: 1,
             tx: 1,
             amount: 1.0,
         });
         Ok(())
     } else {
         return Err(From::from(
             "expected at least two records but got none"));
     }
}
