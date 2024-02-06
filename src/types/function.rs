use serde::Deserialize;

use crate::{Branch, Region};


/// Metrics relate to one specific function.
#[derive(Debug, PartialEq, Deserialize)]
pub struct FunctionMetrics<'a> {
    /// Absolute file pathes indexed via the file ids in `branches` and `regions` attrbiute.
    pub filenames: Vec<&'a str>,
    /// Source code level branches in this function. File ids of the branches are indices into
    /// the `filenames` array.
    pub branches: Vec<Branch>,
    /// Number of times this function was called.
    pub count: u64,
    /// Name of the function.
    pub name: &'a str,
    /// The regions in this function. File ids of the regions are indices into
    /// the `filenames` array.
    pub regions: Vec<Region>,
}