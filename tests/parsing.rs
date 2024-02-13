use std::fs;

use llvm_cov_json::{CoverageReport};

/// Report exported via `llvm-cov export --format=text --instr-profile ...` with LLVM 17.0.4
/// which uses the JSON export format v2.0.1.
static FULL_EXPORT_V2_0_1_PATH: &str = "tests/test_data/full_export_v2_0_1.json";

fn read_json_report(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn calculate_num_branches_via_files_summary(report: &CoverageReport) -> u64 {
    let mut branches = 0;
    for file in report.data[0].files.iter() {
        branches += file.summary.branches.count;
    }
    branches
}

// fn calculate_num_branches_via_files_branches(report: &CoverageReport) -> u64 {
//     let mut branches = 0;
//     for function in report.data[0].functions.iter() {
//         branches += function.branches.len() as u64;
//         // for branch in function.branches.iter() {
//         //     assert_eq!(branch.region_kind, RegionKind::Branch);
//         //     // Check if the branch was expanded in the same file we are currently
//         //     // iterating.
//         //     // let branch_fname = function.filenames[branch.expanded_file_id as usize];
//         //     // assert_eq!(branch_fname, function.filenames);
//         //     branches += 1;
//         // }
//     }
//     branches * 2
// }

#[test]
fn parse_1() {
    let json_data = read_json_report(FULL_EXPORT_V2_0_1_PATH);
    let report: CoverageReport = serde_json::from_str(&json_data).unwrap();
    assert_eq!(report.version, "2.0.1");
    assert_eq!(report.export_type, "llvm.coverage.json.export");
}

#[test]
fn parse_1_branch_counts_via_files_summary() {
    let json_data = read_json_report(FULL_EXPORT_V2_0_1_PATH);
    let report: CoverageReport = serde_json::from_str(&json_data).unwrap();

    let summary_branch_count = report.data[0].summary.branches.count;
    let calculate_branch_cont = calculate_num_branches_via_files_summary(&report);
    assert_eq!(summary_branch_count, calculate_branch_cont);
}
