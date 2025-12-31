mod meta;
mod program;
mod read_group;
mod ref_seq;

use crate::header::{Header, HeaderMeta, Program, ReadGroup, ReferenceSeq};
use logos::Logos;
use std::{collections::HashMap, str::FromStr};

#[derive(Logos, Debug)]
#[logos(skip r"\n+")]
pub(crate) enum HeaderToken<'so> {
    #[token("@", priority = 10)]
    At,
    #[regex(r"HD|SQ|RG|PG", |lex| RecordCode::from_str(lex.slice()), priority = 10)]
    RecordCode(RecordCode),
    #[token(":", priority = 10)]
    Colon,
    #[token(r"\t")]
    Tab,
    #[regex(r"[A-Za-z][A-Za-z0-9]:[^\t\n]+", |lex| lex.slice().split_once(':').unwrap())]
    Field((&'so str /* key */, &'so str /* value */)),
    #[regex(r"CO\t.*", |lex| lex.slice().split_at(3).1, allow_greedy = true)]
    Comment(&'so str),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice())]
    Version(&'so str),
}

#[derive(Debug)]
enum RecordCode {
    // HD
    Meta,
    // SQ
    RefSeq,
    // RG
    ReadGroup,
    // PG
    Program,
}

impl FromStr for RecordCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HD" => Ok(Self::Meta),
            "SQ" => Ok(Self::RefSeq),
            "RG" => Ok(Self::ReadGroup),
            "PG" => Ok(Self::Program),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingPrefix,
    BadRecordCode,
    MissingFieldTag,
    UnknownTag,
    RepeatTag,
    BadVersion,
    MissingVersion,
    MissingValue,
    UnknownValue,
    InvalidUTF8,
    MissingRefSeqName,
    MissingRefSeqLen,
    DuplicateKey,
    MissingRadGroupId,
    MissingProgramId,
}

enum HeaderRow<'so> {
    Meta(HeaderMeta<'so>),
    RefSeq(ReferenceSeq<'so>),
    ReadGroup(ReadGroup<'so>),
    Program(Program<'so>),
    Comment(&'so str),
}

enum HeaderRowKind {
    // HD
    Meta,
    // SQ
    RefSeq,
    // RG
    ReadGroup,
    // PG
    Program,
    // CO
    Comment,
}

pub(crate) fn parse<'so>(s: &'so str) -> Result<Header<'so>, ParseError> {
    let mut meta = None;
    let mut reference_seqs = HashMap::new();
    let mut read_groups = HashMap::new();
    let mut programs = HashMap::new();
    let mut comments = Vec::new();

    for line in s.lines() {
        let header_row = parse_header_row(line.as_bytes())?;
        match header_row {
            HeaderRow::Meta(m) => try_insert_once(&mut meta, m)?,
            HeaderRow::RefSeq(ref_seq) => {
                if reference_seqs.insert(ref_seq.name, ref_seq).is_some() {
                    return Err(ParseError::DuplicateKey);
                }
            }
            HeaderRow::ReadGroup(read_group) => {
                if read_groups.insert(read_group.id, read_group).is_some() {
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
    }
    Ok(Header {
        meta,
        reference_seqs,
        read_groups,
        programs,
        comments,
    })
}

fn parse_header_row(mut s: &[u8]) -> Result<HeaderRow<'_>, ParseError> {
    eat_prefix(&mut s)?;
    let row_kind = parse_header_row_kind(&mut s)?;
    parse_header_row_value(row_kind, &mut s)
}

fn parse_header_row_kind(s: &mut &[u8]) -> Result<HeaderRowKind, ParseError> {
    const KIND_LEN: usize = 2;

    if s.len() < KIND_LEN {
        return Err(ParseError::BadRecordCode);
    }
    let (kind, rest) = s.split_at(KIND_LEN);
    *s = rest;
    match kind {
        b"HD" => Ok(HeaderRowKind::Meta),
        b"SQ" => Ok(HeaderRowKind::RefSeq),
        b"RG" => Ok(HeaderRowKind::ReadGroup),
        b"PG" => Ok(HeaderRowKind::Program),
        b"CO" => Ok(HeaderRowKind::Comment),
        _ => Err(ParseError::BadRecordCode),
    }
}

fn parse_header_row_value<'so>(
    kind: HeaderRowKind,
    s: &mut &'so [u8],
) -> Result<HeaderRow<'so>, ParseError> {
    match kind {
        HeaderRowKind::Meta => meta::parse_meta(s).map(HeaderRow::Meta),
        HeaderRowKind::RefSeq => ref_seq::parse_ref_seq(s).map(HeaderRow::RefSeq),
        HeaderRowKind::ReadGroup => read_group::parse_read_group(s).map(HeaderRow::ReadGroup),
        HeaderRowKind::Program => program::parse_program(s).map(HeaderRow::Program),
        HeaderRowKind::Comment => parse_comment(s).map(HeaderRow::Comment),
    }
}

fn try_insert_once<T>(opt: &mut Option<T>, value: T) -> Result<(), ParseError> {
    match opt.replace(value) {
        Some(_) => Err(ParseError::RepeatTag),
        None => Ok(()),
    }
}

fn parse_comment<'so>(s: &mut &'so [u8]) -> Result<&'so str, ParseError> {
    // Comments can contain \t ? Assume comment goes until the end of line
    let comment = str::from_utf8(s).map_err(|_| ParseError::InvalidUTF8)?;
    *s = b"";
    Ok(comment)
}

fn parse_tag<'a>(s: &mut &'a [u8]) -> Result<&'a [u8], ParseError> {
    const TAG_LEN: usize = 2;
    if s.len() < TAG_LEN {
        return Err(ParseError::MissingFieldTag);
    }
    let (tag, rest) = s.split_at(2);
    *s = rest;
    Ok(tag)
}

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

fn parse_value<'so>(s: &mut &'so [u8]) -> Result<&'so [u8], ParseError> {
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

fn parse_str<'so>(s: &mut &'so [u8]) -> Result<&'so str, ParseError> {
    let value = parse_value(s)?;
    str::from_utf8(value).map_err(|_| ParseError::InvalidUTF8)
}
