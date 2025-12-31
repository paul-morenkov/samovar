use crate::header::{
    Platform, ReadGroup,
    parser::{
        ParseError, eat_field_delimiter, eat_kv_separator, parse_str, parse_tag, parse_value,
        try_insert_once,
    },
};

pub(super) fn parse_read_group<'so>(s: &mut &'so [u8]) -> Result<ReadGroup<'so>, ParseError> {
    let mut id = None;
    let mut barcode = None;
    let mut center = None;
    let mut description = None;
    let mut date = None;
    let mut flow_order = None;
    let mut key_sequence = None;
    let mut library = None;
    let mut programs = None;
    let mut insert_size = None;
    let mut platform = None;
    let mut platform_model = None;
    let mut platform_unit = None;
    let mut sample = None;

    while !s.is_empty() {
        eat_field_delimiter(s)?;
        let tag = parse_tag(s)?;
        eat_kv_separator(s)?;
        // TODO: fill in meta fields
        match tag {
            b"ID" => try_insert_once(&mut id, parse_str(s)?)?,
            b"BC" => try_insert_once(&mut barcode, parse_str(s)?)?,
            b"CN" => try_insert_once(&mut center, parse_str(s)?)?,
            b"DS" => try_insert_once(&mut description, parse_str(s)?)?,
            b"DT" => try_insert_once(&mut date, parse_str(s)?)?,
            b"FO" => try_insert_once(&mut flow_order, parse_str(s)?)?,
            b"KS" => try_insert_once(&mut key_sequence, parse_str(s)?)?,
            b"LB" => try_insert_once(&mut library, parse_str(s)?)?,
            // TODO: unclear what format the PG field is in
            b"PG" => try_insert_once(&mut programs, parse_str(s)?)?,
            b"PI" => try_insert_once(
                &mut insert_size,
                parse_str(s)?
                    .parse()
                    .map_err(|_| ParseError::UnknownValue)?,
            )?,
            b"PL" => try_insert_once(&mut platform, parse_plaform(s)?)?,
            b"PM" => try_insert_once(&mut platform_model, parse_str(s)?)?,
            b"PU" => try_insert_once(&mut platform_unit, parse_str(s)?)?,
            b"SM" => try_insert_once(&mut sample, parse_str(s)?)?,

            _ => return Err(ParseError::UnknownTag),
        }
    }

    Ok(ReadGroup {
        id: id.ok_or(ParseError::MissingRadGroupId)?,
        barcode,
        center,
        description,
        date,
        flow_order,
        key_sequence,
        library,
        programs,
        insert_size,
        platform,
        platform_model,
        platform_unit,
        sample,
    })
}

fn parse_plaform(s: &mut &[u8]) -> Result<Platform, ParseError> {
    let value = parse_value(s)?;
    match value {
        b"CAPILLARY" => Ok(Platform::Capillary),
        b"DNBSEQ" => Ok(Platform::Dnbseq),
        b"ELEMENT" => Ok(Platform::Element),
        b"HELICOS" => Ok(Platform::Helicos),
        b"ILLUMINA" => Ok(Platform::Illumina),
        b"IONTORRENT" => Ok(Platform::Iontorent),
        b"LS454" => Ok(Platform::LS454),
        b"ONT" => Ok(Platform::Ont),
        b"PACBIO" => Ok(Platform::Pacbio),
        b"SINGULAR" => Ok(Platform::Singular),
        b"SOLID" => Ok(Platform::Solid),
        b"ULTIMA" => Ok(Platform::Ultima),
        _ => Err(ParseError::UnknownValue),
    }
}
