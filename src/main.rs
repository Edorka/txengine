use std::error::Error;
use std::fs::File;

mod operations;
use crate::operations::{Operation, for_each_operation_in};
 

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let file_path = &args[1];
    let source = File::open(file_path)?;
    for_each_operation_in(source, |item: Operation| {println!("{:?}", item)});
    Ok(())
}
