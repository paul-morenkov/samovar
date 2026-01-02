use std::{collections::HashMap, io::BufRead};

use crate::header::{
    Header,
    parser::{HeaderRow, ParseError, parse_header_row, try_insert_once},
};

pub fn read_header(reader: &mut impl BufRead) -> Result<Header, ParseError> {
    let mut meta = None;
    let mut reference_seqs = HashMap::new();
    let mut read_groups = HashMap::new();
    let mut programs = HashMap::new();
    let mut comments = Vec::new();

    let mut buf = Vec::new();
    // TODO: improve error handling
    while reader
        .read_until(b'\n', &mut buf)
        .map_err(|_| ParseError::IOError)?
        > 0
    {
        // Remove the newline to match functionality of String::lines()
        let header_row = parse_header_row(&buf[..buf.len() - 1])?;
        match header_row {
            HeaderRow::Meta(m) => try_insert_once(&mut meta, m)?,
            HeaderRow::RefSeq(ref_seq) => {
                if reference_seqs
                    .insert(ref_seq.name.clone(), ref_seq)
                    .is_some()
                {
                    return Err(ParseError::DuplicateKey);
                }
            }
            HeaderRow::ReadGroup(read_group) => {
                if read_groups
                    .insert(read_group.id.clone(), read_group)
                    .is_some()
                {
                    return Err(ParseError::DuplicateKey);
                }
            }
            HeaderRow::Program(program) => {
                if programs.insert(program.id.clone(), program).is_some() {
                    return Err(ParseError::DuplicateKey);
                }
            }
            HeaderRow::Comment(comment) => comments.push(comment),
        }
        buf.clear();
    }
    Ok(Header {
        meta,
        reference_seqs,
        read_groups,
        programs,
        comments,
    })
}
