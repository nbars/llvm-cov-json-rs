use serde::Deserialize;

use crate::{Branch, Expansion};

use super::Summary;

//dict_keys(['branches', 'expansions', 'filename', 'segments', 'summary'])

/// Summary of the regions metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct FileMetrics<'a> {
    /// Absolute path ot the file.
    pub filename: &'a str,
    /// Summary of all metrecis.
    pub summary: Summary,
    /// Branches in this file.
    pub branches: Vec<Branch>,
    /// The segments of the file. Each segment starts at some point in the source
    /// file, and is "valid" until the next segment start is encountered.
    pub segments: Vec<Segment>,
    /// Expansion of this file. May be None, if the export of expansions have been disabled.
    pub expansions: Option<Vec<Expansion<'a>>>,
}

/// Execution count information starting at a point (row and column) in a file.
/// A sequence of CoverageSegments gives execution counts for a file in a format.
/// See the [LLVM sourceode](https://github.com/llvm/llvm-project/blob/bd611264993f64decbce178d460caf1d1cb05f59/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L448)
/// for mor information.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Segment {
    /// Source code line this segment starts.
    line: u64,
    /// Colum this segment starts.
    col: u64,
    /// Number of times this segment was executed.
    count: u64,
    /// If false, the segment was not instrumented or skipped.
    has_count: bool,
    /// Whether this enters a new region.
    /// TODO: Needs documentation.
    is_region_entry: bool,
    /// Whether this enters a gap region.
    /// TODO: Needs documentation.
    is_gap_region: bool,
}
