mod alignment;
mod header;

use header::Header;
use std::fs;
use std::ops::Deref;

use crate::header::parser::ParseError;

fn main() -> Result<(), ParseError> {
    let sam = fs::read_to_string("examples/header.sam").unwrap();
    let header = Header::try_from(sam.deref())?;
    dbg!(header);
    Ok(())
}
