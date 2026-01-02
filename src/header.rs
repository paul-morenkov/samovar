use std::{collections::HashMap, str::FromStr};

use crate::header::parser::ParseError;

pub mod parser;
pub mod reader;

#[derive(Debug, Default)]
pub(crate) struct Header {
    meta: Option<HeaderMeta>,
    reference_seqs: HashMap<String, ReferenceSeq>,
    read_groups: HashMap<String, ReadGroup>,
    programs: HashMap<ProgramID, Program>,
    comments: Vec<String>,
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parser::parse(value)
    }
}

#[derive(Debug, Default)]
struct HeaderMeta {
    // VN
    format_version: Version,
    // SO
    alignment_sort_order: Option<SortOrder>,
    // GO
    alignment_grouping: Option<AlignmentGrouping>,
    // SS
    alignment_sub_sorting: Option<String>,
}
#[derive(Debug, Default)]
struct Version {
    major: usize,
    minor: usize,
}

#[derive(Debug, Default)]
enum SortOrder {
    #[default]
    Unknown,
    Unsorted,
    QueryName,
    Coordinate,
}

#[derive(Debug, Default)]
enum AlignmentGrouping {
    #[default]
    None,
    Query,
    Reference,
}

#[derive(Debug, Default)]
struct ReferenceSeq {
    // SN
    name: String,
    // LN
    length: u64,
    // AH
    alternate_locus: Option<String>,
    // AN
    alternate_names: Option<Vec<String>>,
    // AS
    assembly_id: Option<String>,
    // DS
    description: Option<String>,
    // M5
    checksum: Option<String>,
    // SP
    species: Option<String>,
    // TP
    topology: Option<Topology>,
    // UR
    uri: Option<String>,
}

#[derive(Debug)]
enum Topology {
    Linear,
    Circular,
}

#[derive(Debug, Default)]
struct ReadGroup {
    // ID
    id: String,
    // BC
    barcode: Option<String>,
    // CN
    center: Option<String>,
    // DS
    description: Option<String>,
    // DT
    date: Option<String>,
    // FO
    flow_order: Option<String>,
    // KS
    key_sequence: Option<String>,
    // LB
    library: Option<String>,
    // PG
    programs: Option<String>,
    // PI
    insert_size: Option<u32>,
    // PL
    platform: Option<Platform>,
    // PM
    platform_model: Option<String>,
    // PU
    platform_unit: Option<String>,
    // SM
    sample: Option<String>,
}

#[derive(Debug)]
enum Platform {
    Capillary,
    Dnbseq,
    Element,
    Helicos,
    Illumina,
    Iontorent,
    LS454,
    Ont,
    Pacbio,
    Singular,
    Solid,
    Ultima,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct ProgramID(String);

#[derive(Debug, Default)]
struct Program {
    // ID
    id: ProgramID,
    // PN
    name: Option<String>,
    // CL
    command_line: Option<String>,
    // PP
    previous: Option<ProgramID>,
    // DS
    description: Option<String>,
    // VN
    version: Option<String>,
}
