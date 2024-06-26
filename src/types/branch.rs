use serde::{Deserialize, Deserializer, de::Error};

use crate::RegionKind;

/// A single branch and associated metrics. According to the [LLVM documentation](https://llvm.org/docs/CoverageMappingFormat.html#advanced-concepts),
/// a branch "may comprise larger boolean expressions using boolean logical operators", thus a the term branch refers to a branch found on source code
/// level, not on binary level.
/// See [CoverageExporterJson.cpp](https://github.com/llvm/llvm-project/blob/309d55140c46384b6de7a7573206cbeba3f7077f/llvm/tools/llvm-cov/CoverageExporterJson.cpp#L93) for details
/// regarding the format.
#[derive(Debug, PartialEq)]
pub struct Branch {
    /// Source line this branch condition starts.
    pub line_start: u64,
    ///  Column of `line_start` this branch starts.
    pub column_start: u64,
    /// Source line where the branch ends.
    pub line_end: u64,
    /// Column of `line_end` where the branch ends.
    pub column_end: u64,
    /// Number of times the true branch was taken.
    pub execution_count: u64,
    /// Number of times the false branch was taken.
    pub false_execution_count: u64,
    /// The file id this branch orginatess from. If this is part of an expansion,
    /// this is, e.g., the header file.
    pub file_id: u64,
    /// The file id where this branch has been expanded to.
    /// I.e., if branches are introduced via a macro, this is the file that
    /// used the macro.
    pub expanded_file_id: u64,
    /// The kind of this region.
    pub region_kind: RegionKind,
}

#[derive(Deserialize)]
struct BranchTuple(u64, u64, u64, u64, u64, u64, u64, u64, u64);

impl<'de> Deserialize<'de> for Branch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let tuple = BranchTuple::deserialize(deserializer)?;
        let region_kind = match RegionKind::try_from(tuple.8) {
            Ok(kind) => kind,
            Err(err) => return Err(Error::custom(err)),
        };

        Ok(Branch {
            line_start: tuple.0,
            column_start: tuple.1,
            line_end: tuple.2,
            column_end: tuple.3,
            execution_count: tuple.4,
            false_execution_count: tuple.5,
            file_id: tuple.6,
            expanded_file_id: tuple.7,
            region_kind,
        })
    }
}