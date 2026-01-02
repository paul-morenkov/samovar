use crate::{
    alignment::{Alignment, Flag},
    header::parser::ParseError,
};

pub(crate) fn parse(s: &str) -> Result<Vec<Alignment>, ParseError> {
    s.lines()
        .map(|line| parse_alignment(line.as_bytes()))
        .collect()
}

pub(crate) fn parse_alignment(s: &[u8]) -> Result<Alignment, ParseError> {
    const DELIM: u8 = b'\t';
    let mut fields = s.split(|&c| c == DELIM);

    fn next_field<'a>(i: &mut impl Iterator<Item = &'a [u8]>) -> Result<&'a str, ParseError> {
        i.next()
            .ok_or(ParseError::MissingAlignmentField)
            .map(parse_str)?
    }

    let query_name = next_field(&mut fields)?.to_owned();
    // FIXME: change error type
    let flag = Flag(
        next_field(&mut fields)?
            .parse()
            .map_err(|_| ParseError::UnknownValue)?,
    );
    let ref_seq_name = next_field(&mut fields)?.to_owned();
    let pos = next_field(&mut fields)?
        .parse()
        .map_err(|_| ParseError::UnknownValue)?;
    let map_quality = next_field(&mut fields)?
        .parse()
        .map_err(|_| ParseError::UnknownValue)?;
    let cigar = next_field(&mut fields)?.to_owned();
    let rnext = next_field(&mut fields)?.to_owned();
    let pnext = next_field(&mut fields)?
        .parse()
        .map_err(|_| ParseError::UnknownValue)?;
    let template_len = next_field(&mut fields)?
        .parse()
        .map_err(|_| ParseError::UnknownValue)?;
    let sequence = next_field(&mut fields)?
        .parse()
        .map_err(|_| ParseError::UnknownValue)?;
    let phred_quality = next_field(&mut fields)?.to_owned();

    // TODO: Parse and return other fields
    // This may be challenging due to different fields having different data types
    // NOTE: Can there be invalid UTF8 here?
    let _other_fields: Vec<_> = fields.map(parse_str).collect::<Result<_, _>>()?;

    Ok(Alignment {
        query_name,
        flag,
        ref_seq_name,
        pos,
        map_quality,
        cigar,
        rnext,
        pnext,
        template_len,
        sequence,
        phred_quality,
    })
}

// FIXME: Move these functions somewhere so that they're not duplicated in `crate::header::parser`
fn eat_prefix(s: &mut &[u8]) -> Result<(), ParseError> {
    const PREFIX: u8 = b'@';
    if let Some((&PREFIX, rest)) = s.split_first() {
        *s = rest;
        Ok(())
    } else {
        Err(ParseError::MissingPrefix)
    }
}

fn eat_field_delimiter(s: &mut &[u8]) -> Result<(), ParseError> {
    const DELIM: u8 = b'\t';
    if let Some((&DELIM, rest)) = s.split_first() {
        *s = rest;
        Ok(())
    } else {
        Err(ParseError::MissingPrefix)
    }
}

fn eat_kv_separator(s: &mut &[u8]) -> Result<(), ParseError> {
    const SEP: u8 = b':';
    if let Some((&SEP, rest)) = s.split_first() {
        *s = rest;
        Ok(())
    } else {
        Err(ParseError::MissingPrefix)
    }
}

fn parse_value<'a>(s: &mut &'a [u8]) -> Result<&'a [u8], ParseError> {
    const DELIM: u8 = b'\t';

    let i = s.iter().position(|&b| b == DELIM).unwrap_or(s.len());
    let (value, rest) = s.split_at(i);
    *s = rest;
    if value.is_empty() {
        Err(ParseError::MissingValue)
    } else {
        Ok(value)
    }
}

fn parse_str(s: &[u8]) -> Result<&str, ParseError> {
    str::from_utf8(s).map_err(|_| ParseError::InvalidUTF8)
}
