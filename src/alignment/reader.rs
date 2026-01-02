use std::io::BufRead;

use crate::{
    alignment::{Alignment, parser::parse_alignment},
    header::parser::ParseError,
};

pub fn read_alignments(r: &mut impl BufRead) -> Result<Vec<Alignment>, ParseError> {
    r.lines()
        .map(|line| {
            line.as_deref()
                .map(read_alignment)
                .map_err(|_| ParseError::IOError)?
        })
        .collect()
}

fn read_alignment(s: &str) -> Result<Alignment, ParseError> {
    parse_alignment(s.as_bytes())
}
