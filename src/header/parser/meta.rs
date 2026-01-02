use crate::header::{
    AlignmentGrouping, HeaderMeta, SortOrder, Version,
    parser::{
        ParseError, eat_field_delimiter, eat_kv_separator, parse_tag, parse_value, try_insert_once,
    },
};

#[derive(Debug)]
pub enum MetaParseError {
    #[non_exhaustive]
    MissingVersion,
    #[non_exhaustive]
    BadVersion,
    #[non_exhaustive]
    MissingFieldDelimiter,
    #[non_exhaustive]
    MissingKeyValueSeparator,
    #[non_exhaustive]
    BadSortOrder,
    #[non_exhaustive]
    BadGrouping,
    #[non_exhaustive]
    BadSubSortOrder,
}

pub(crate) fn parse_meta(s: &mut &[u8]) -> Result<HeaderMeta, ParseError> {
    let mut version = None;
    let mut sort_order = None;
    let mut grouping = None;
    let mut sub_sorting = None;

    while !s.is_empty() {
        eat_field_delimiter(s)?;
        let tag = parse_tag(s)?;
        eat_kv_separator(s)?;
        // TODO: fill in meta fields
        match tag {
            b"VN" => try_insert_once(&mut version, parse_version(s)?)?,
            b"SO" => try_insert_once(&mut sort_order, parse_sort_order(s)?)?,
            b"GO" => try_insert_once(&mut grouping, parse_grouping(s)?)?,
            b"SS" => try_insert_once(&mut sub_sorting, parse_sub_sorting(s)?)?,
            _ => return Err(ParseError::UnknownTag),
        };
    }
    Ok(HeaderMeta {
        format_version: version.ok_or(ParseError::MissingVersion)?,
        alignment_sort_order: sort_order,
        alignment_grouping: grouping,
        alignment_sub_sorting: sub_sorting,
    })
}

fn parse_version(s: &mut &[u8]) -> Result<Version, ParseError> {
    let value = parse_value(s)?;

    if let Some(i) = value.iter().position(|&c| c == b'.') {
        let (major, minor) = value.split_at(i);
        let major = str::from_utf8(major)
            .map_err(|_| ParseError::InvalidUTF8)?
            .parse()
            .map_err(|_| ParseError::BadVersion)?;
        let minor = str::from_utf8(&minor[1..])
            .map_err(|_| ParseError::InvalidUTF8)?
            .parse()
            .map_err(|_| ParseError::BadVersion)?;
        Ok(Version { major, minor })
    } else {
        Err(ParseError::BadVersion)
    }
}

fn parse_sort_order(s: &mut &[u8]) -> Result<SortOrder, ParseError> {
    let sort_order = parse_value(s)?;

    match sort_order {
        b"unknown" => Ok(SortOrder::Unknown),
        b"unsorted" => Ok(SortOrder::Unsorted),
        b"queryname" => Ok(SortOrder::QueryName),
        b"coordinate" => Ok(SortOrder::Coordinate),
        _ => Err(ParseError::UnknownValue),
    }
}

fn parse_grouping(s: &mut &[u8]) -> Result<AlignmentGrouping, ParseError> {
    let grouping = parse_value(s)?;
    match grouping {
        b"none" => Ok(AlignmentGrouping::None),
        b"query" => Ok(AlignmentGrouping::Query),
        b"reference" => Ok(AlignmentGrouping::Reference),
        _ => Err(ParseError::UnknownValue),
    }
}
fn parse_sub_sorting(s: &mut &[u8]) -> Result<String, ParseError> {
    // TODO: figure out how UTF8 handling should happen
    parse_value(s).map(|b| String::from_utf8(b.to_owned()).map_err(|_| ParseError::InvalidUTF8))?
}
