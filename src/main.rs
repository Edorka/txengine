use std::error::Error;
use std::fs::File;

mod transactions;
use crate::transactions::{Transaction, for_each_transaction_in};
mod accounts;
mod state;
 

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let source = File::open(file_path)?;
    for_each_transaction_in(source, |item: Transaction| {println!("{:?}", item)});
    Ok(())
}
