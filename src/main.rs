mod alignment;
mod header;

use std::{fs, io::BufReader};

use header::{Header, parser::ParseError, reader::read_header};

fn main() -> Result<(), ParseError> {
    let sam = fs::read_to_string("examples/header.sam").unwrap();
    let header = Header::try_from(sam)?;
    dbg!(header);

    let f = fs::File::open("examples/header.sam").unwrap();
    let mut reader = BufReader::new(f);
    let header = read_header(&mut reader)?;
    dbg!(header);
    Ok(())

}
