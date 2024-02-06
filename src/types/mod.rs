
use serde::Deserialize;

mod region;
pub use region::*;

mod branch;
pub use branch::*;

mod expansion;
pub use expansion::*;

mod function;
pub use function::*;

mod file;
pub use file::*;

mod summary;
pub use summary::*;


#[derive(Debug, PartialEq, Deserialize)]
pub struct CoverageReport<'a> {
    /// The version of the exported data.
    pub version: &'a str,
    /// The type of exported data. Must be set to llvm.coverage.json.export.
    #[serde(rename = "type")]
    pub export_type: &'a str,
    /// This is the actual coverage data. According to the [LLVM source code](https://github.com/llvm/llvm-project/blob/309d55140c46384b6de7a7573206cbeba3f7077f/llvm/tools/llvm-cov/CoverageExporterJson.cpp#L17C22-L17C69)
    /// this is a "homogeneous array of one or more export objects", however, in practice, it contains only one element.
    pub data: Vec<ExportObject<'a>>,
}

impl<'a> CoverageReport<'a> {

    pub fn from_str(s: &str) -> Result<CoverageReport, serde_json::Error> {
        Ok(serde_json::from_str(s)?)
    }

}


#[derive(Debug, PartialEq, Deserialize)]
pub struct ExportObject<'a> {
    /// Summary of all coverage data in this export.
    #[serde(rename = "totals")]
    pub summary: Summary,
    /// Per file metrics.
    #[serde(borrow)]
    pub files: Vec<FileMetrics<'a>>,
    #[serde(borrow)]
    pub functions: Vec<FunctionMetrics<'a>>,
}
