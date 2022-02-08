use std::error::Error;
use std::fs::File;

mod transactions;
use crate::transactions::Transaction;
mod accounts;
mod io;
mod state;
use crate::io::{for_each_item_in, output};
use crate::state::State;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let source = File::open(file_path)?;
    let mut state = State::default();
    for_each_item_in(source, |transaction: Transaction| {
        state.perform(&transaction).ok();
    });
    output(state.get_results().iter());
    Ok(())
}
