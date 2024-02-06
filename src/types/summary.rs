use serde::Deserialize;

/// Summary of the different metrics.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Summary {
    /// Summary of all branches.
    pub branches: BranchSummary,
    /// Summary of all functions.
    pub functions: FunctionSummary,
    /// Summary of all instantiations.
    pub instantiations: InstantiationSummary,
    /// Summary of all lines of code.
    pub lines: LineSummary,
    /// Summary of all regions.
    pub regions: RegionSummary,
}

/// Summary of the branches metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct BranchSummary {
    /// Total number of branches that cloud potentially be covered,
    pub count: u64,
    /// Number of covered branches.
    pub covered: u64,
    /// Number of the branches that have not been covered.
    pub notcovered: u64,
    /// The fraction of branches covered in percent.
    pub percent: f32,
}

/// Summary of the functions metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct FunctionSummary {
    /// Total number of function.
    pub count: u64,
    /// Number of functions covered.
    pub covered: u64,
    /// Fraction of functions that have been covered in percent.
    pub percent: f32,
}

/// Summary of the instations metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct InstantiationSummary {
    /// Number of instations, i.e., total count of macros that have been instantiated.
    pub count: u64,
    /// Number of macro instantiations that have been covered.
    pub covered: u64,
    /// Fraction of instantiations that have been covered in percent.
    pub percent: f32,
}

/// Summary of the lines metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct LineSummary {
    /// Number of lines that could be covered.
    pub count: u64,
    /// Number of lines that have been covered.
    pub covered: u64,
    /// Fraction of lines that have been covered in percent.
    pub percent: f32,
}

/// Summary of the regions metric.
#[derive(Debug, PartialEq, Deserialize)]
pub struct RegionSummary {
    /// Number of total regions.
    pub count: u64,
    /// Number of covered regions.
    pub covered: u64,
    /// Number of the regions that have not been covered.
    pub notcovered: u64,
    /// Fraction of regions that have been covered in percent.
    pub percent: f32,
}
