use std::collections::HashMap;

pub(crate) mod parser;

#[derive(Debug, Default)]
pub(crate) struct Header<'so> {
    meta: Option<HeaderMeta<'so>>,
    reference_seqs: HashMap<&'so str, ReferenceSeq<'so>>,
    read_groups: HashMap<&'so str, ReadGroup<'so>>,
    programs: HashMap<ProgramID<'so>, Program<'so>>,
    comments: Vec<&'so str>,
}

impl<'so> TryFrom<&'so str> for Header<'so> {
    type Error = parser::ParseError;

    fn try_from(value: &'so str) -> Result<Self, Self::Error> {
        parser::parse(value)
    }
}

#[derive(Debug, Default)]
struct HeaderMeta<'so> {
    // VN
    format_version: Version,
    // SO
    alignment_sort_order: Option<SortOrder>,
    // GO
    alignment_grouping: Option<AlignmentGrouping>,
    // SS
    alignment_sub_sorting: Option<&'so str>,
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
struct ReferenceSeq<'so> {
    // SN
    name: &'so str,
    // LN
    length: u64,
    // AH
    alternate_locus: Option<&'so str>,
    // AN
    alternate_names: Option<Vec<&'so str>>,
    // AS
    assembly_id: Option<&'so str>,
    // DS
    description: Option<&'so str>,
    // M5
    checksum: Option<&'so str>,
    // SP
    species: Option<&'so str>,
    // TP
    topology: Option<Topology>,
    // UR
    uri: Option<&'so str>,
}

#[derive(Debug)]
enum Topology {
    Linear,
    Circular,
}

#[derive(Debug, Default)]
struct ReadGroup<'so> {
    // ID
    id: &'so str,
    // BC
    barcode: Option<&'so str>,
    // CN
    center: Option<&'so str>,
    // DS
    description: Option<&'so str>,
    // DT
    date: Option<&'so str>,
    // FO
    flow_order: Option<&'so str>,
    // KS
    key_sequence: Option<&'so str>,
    // LB
    library: Option<&'so str>,
    // PG
    programs: Option<&'so str>,
    // PI
    insert_size: Option<u32>,
    // PL
    platform: Option<Platform>,
    // PM
    platform_model: Option<&'so str>,
    // PU
    platform_unit: Option<&'so str>,
    // SM
    sample: Option<&'so str>,
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
struct ProgramID<'so>(&'so str);

#[derive(Debug, Default)]
struct Program<'so> {
    // ID
    id: ProgramID<'so>,
    // PN
    name: Option<&'so str>,
    // CL
    command_line: Option<&'so str>,
    // PP
    previous: Option<ProgramID<'so>>,
    // DS
    description: Option<&'so str>,
    // VN
    version: Option<&'so str>,
}
