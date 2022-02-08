use csv;
use csv::{ReaderBuilder, Trim};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io;
use std::slice::Iter;

pub fn for_each_item_in<D: DeserializeOwned, F: FnMut(D)>(
    source: impl io::Read,
    mut do_for_each: F,
) {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .flexible(true)
        .trim(Trim::All)
        .from_reader(source);
    for record in rdr.deserialize() {
        do_for_each(record.unwrap());
    }
}

pub fn output<F>(source: Iter<F>)
where
    F: Serialize,
{
    let mut wtr = csv::WriterBuilder::new().from_writer(io::stdout());
    for item in source {
        wtr.serialize(item).ok();
    }
}
