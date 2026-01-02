mod alignment;
mod header;

use std::{fs, io::BufReader, str::FromStr};

use header::{Header, parser::ParseError, reader::read_header};
use alignment::reader::read_alignments;

fn main() -> Result<(), ParseError> {
    let sam = fs::read_to_string("examples/header.sam").unwrap();
    let header = Header::from_str(&sam)?;
    dbg!(header);

    let f = fs::File::open("examples/header.sam").unwrap();
    let mut reader = BufReader::new(f);
    let header = read_header(&mut reader)?;
    dbg!(header);

    let f = fs::File::open("examples/alignments.sam").unwrap();
    let mut reader = BufReader::new(f);
    let alignments = read_alignments(&mut reader)?;
    dbg!(alignments);
    Ok(())
}
