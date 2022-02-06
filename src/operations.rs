use serde::{Deserialize};
use csv::{ReaderBuilder, Trim};
use std::io::Read;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Deposit,
    Withdrawal
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub struct Operation {
    #[serde(rename="type")]
    op_type: OperationType,
    client: u16,
    tx: u32,
    amount: f32,
}

pub fn for_each_operation_in(source: impl Read, do_for_each: fn(Operation)) {
     let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .trim(Trim::All)
        .from_reader(source);//std::io::stdin());
     for record in rdr.deserialize() {
         do_for_each(record.unwrap());
     }
} 
