use crate::header::{
    Program, ProgramID,
    parser::{
        ParseError, eat_field_delimiter, eat_kv_separator, parse_str, parse_tag, try_insert_once,
    },
};

pub(super) fn parse_program(s: &mut &[u8]) -> Result<Program, ParseError> {
    let mut id = None;
    let mut name = None;
    let mut command_line = None;
    let mut previous = None;
    let mut description = None;
    let mut version = None;

    while !s.is_empty() {
        eat_field_delimiter(s)?;
        let tag = parse_tag(s)?;
        eat_kv_separator(s)?;
        // TODO: fill in meta fields
        match tag {
            b"ID" => try_insert_once(&mut id, parse_str(s).map(|s| ProgramID(s.into()))?)?,
            b"PN" => try_insert_once(&mut name, parse_str(s)?.into())?,
            b"CL" => try_insert_once(&mut command_line, parse_str(s)?.into())?,
            b"PP" => try_insert_once(&mut previous, parse_str(s).map(|s| ProgramID(s.into()))?)?,
            b"DS" => try_insert_once(&mut description, parse_str(s)?.into())?,
            b"VN" => try_insert_once(&mut version, parse_str(s)?.into())?,
            _ => return Err(ParseError::UnknownTag),
        };
    }
    Ok(Program {
        id: id.ok_or(ParseError::MissingProgramId)?,
        name,
        command_line,
        previous,
        description,
        version,
    })
}
