use crate::header::{
    ReferenceSeq, Topology,
    parser::{
        ParseError, eat_field_delimiter, eat_kv_separator, parse_str, parse_tag, parse_value,
        try_insert_once,
    },
};

pub(super) fn parse_ref_seq(s: &mut &[u8]) -> Result<ReferenceSeq, ParseError> {
    let mut name = None;
    let mut len = None;
    let mut alt_locus = None;
    let mut alt_names = None;
    let mut assembly_id = None;
    let mut description = None;
    let mut checksum = None;
    let mut species = None;
    let mut topology = None;
    let mut uri = None;

    while !s.is_empty() {
        eat_field_delimiter(s)?;
        let tag = parse_tag(s)?;
        eat_kv_separator(s)?;
        // TODO: fill in meta fields
        match tag {
            b"SN" => try_insert_once(&mut name, parse_str(s)?.into())?,
            b"LN" => try_insert_once(&mut len, parse_len(s)?)?,
            b"AH" => try_insert_once(&mut alt_locus, parse_str(s)?.into())?,
            b"AN" => try_insert_once(&mut alt_names, parse_alt_names(s)?)?,
            b"AS" => try_insert_once(&mut assembly_id, parse_str(s)?.into())?,
            b"DS" => try_insert_once(&mut description, parse_str(s)?.into())?,
            b"M5" => try_insert_once(&mut checksum, parse_str(s)?.into())?,
            b"SP" => try_insert_once(&mut species, parse_str(s)?.into())?,
            b"TP" => try_insert_once(&mut topology, parse_topology(s)?)?,
            b"UR" => try_insert_once(&mut uri, parse_str(s)?.into())?,

            _ => return Err(ParseError::UnknownTag),
        };
    }

    Ok(ReferenceSeq {
        name: name.ok_or(ParseError::MissingRefSeqName)?,
        length: len.ok_or(ParseError::MissingRefSeqLen)?,
        alternate_locus: alt_locus,
        alternate_names: alt_names,
        assembly_id,
        description,
        checksum,
        species,
        topology,
        uri,
    })
}

fn parse_len(s: &mut &[u8]) -> Result<u64, ParseError> {
    let value = parse_str(s)?;
    value.parse().map_err(|_| ParseError::UnknownValue)
}

fn parse_alt_names(s: &mut &[u8]) -> Result<Vec<String>, ParseError> {
    let value = parse_str(s)?;
    Ok(value.split(',').map(str::to_owned).collect())
}

fn parse_topology(s: &mut &[u8]) -> Result<Topology, ParseError> {
    let value = parse_value(s)?;
    match value {
        b"linear" => Ok(Topology::Linear),
        b"circular" => Ok(Topology::Circular),
        _ => Err(ParseError::UnknownValue),
    }
}
