pub mod parser;
pub mod reader;

#[derive(Debug)]
pub(crate) struct Alignment {
    query_name: String,
    flag: Flag,
    ref_seq_name: String,
    pos: u32,
    map_quality: u8,
    cigar: String,
    rnext: String,
    pnext: u32,
    template_len: i32,
    sequence: String,
    phred_quality: String,
}

#[derive(Debug)]
struct Flag(u16);

impl Flag {
    const fn has_multiple_segments(&self) -> bool {
        self.0 & 0x1 > 0
    }
    const fn each_seg_aligned(&self) -> bool {
        self.0 & 0x2 > 0
    }
    const fn is_unmapped(&self) -> bool {
        self.0 & 0x4 > 0
    }
    const fn next_is_unmapped(&self) -> bool {
        self.0 & 0x8 > 0
    }
    const fn is_reverse_complement(&self) -> bool {
        self.0 & 0x10 > 0
    }
    const fn next_is_reverse_complement(&self) -> bool {
        self.0 & 0x20 > 0
    }
    const fn is_first_segment(&self) -> bool {
        self.0 & 0x40 > 0
    }
    const fn is_last_segment(&self) -> bool {
        self.0 & 0x80 > 0
    }
    const fn is_secondary_alignment(&self) -> bool {
        self.0 & 0x100 > 0
    }
    const fn not_passing_filters(&self) -> bool {
        self.0 & 0x200 > 0
    }
    const fn is_duplicate(&self) -> bool {
        self.0 & 0x400 > 0
    }
    const fn is_supplementary_alignment(&self) -> bool {
        self.0 & 0x800 > 0
    }
    const fn is_primary_line(&self) -> bool {
        self.0 & 0x900 == 0
    }
}
