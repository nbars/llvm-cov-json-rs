use serde::Deserialize;

use crate::{Branch, Region};

/// Metrics of an expansion, i.e., a expanded macro or include statement.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Expansion<'a> {
    #[serde(borrow)]
    /// Vector that is references via index by the branches, source_region and target_regions attribute
    /// in order to refer to a specific file.
    pub filenames: Vec<&'a str>,
    /// All branches in this expansion.
    pub branches: Vec<Branch>,
    /// The source and destination of the expansion (i.e., macro/include statement).
    /// Indexing the `filenames` array with the `source_region.file_id` returns the
    /// path of the file in which the macro/include was used. The `source_region.line_*`
    /// and `source_region.column_*` attrbiutes refer to the code location where in macro was used.
    /// The `source_region.expanded_file_id` can be used as index for the `filenames` array to
    /// get the file path where the expanded macro or included file is located.
    pub source_region: Region,
    pub target_regions: Vec<Region>,
}