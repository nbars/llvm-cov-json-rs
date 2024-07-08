use std::fs;

use llvm_cov_json::CoverageReport;

/// Report exported via `llvm-cov export --format=text --instr-profile ...` with LLVM 17.0.4
/// which uses the JSON export format v2.0.1.
static FULL_EXPORT_V2_0_1_PATH: &str = "tests/test_data/mosquitto_full_export_v2_0_1.json";
static WO_EXPANSIONS_EXPORT_V2_0_1_PATH: &str = "tests/test_data/mosquitto_wo_expansions_export_v2_0_1.json";

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
    let calculated_branch_cont = calculate_num_branches_via_files_summary(&report);
    assert_eq!(summary_branch_count, calculated_branch_cont);
}


#[test]
fn compare_full_report_vs_wo_expansions() {
    let json_data = read_json_report(FULL_EXPORT_V2_0_1_PATH);
    let full_report: CoverageReport = serde_json::from_str(&json_data).unwrap();

    let json_data = read_json_report(WO_EXPANSIONS_EXPORT_V2_0_1_PATH);
    let report_wo_expansions: CoverageReport = serde_json::from_str(&json_data).unwrap();
    report_wo_expansions.data[0].files.iter().for_each(|e| assert!(e.expansions.is_none()));

    // compare branches based on summary
    assert_eq!(full_report.data[0].summary.branches, report_wo_expansions.data[0].summary.branches);

    // comapre branches based on files array
    let branches_full: usize = full_report.data[0].files.iter().map(|f| f.branches.len()).sum();
    let branches_wo_exp: usize = report_wo_expansions.data[0].files.iter().map(|f| f.branches.len()).sum();
    assert_eq!(branches_full, branches_wo_exp);

    // comapre branches based on functions array
    let branches_full: usize = full_report.data[0].functions.iter().map(|f| f.branches.len()).sum();
    let branches_wo_exp: usize = report_wo_expansions.data[0].functions.iter().map(|f| f.branches.len()).sum();
    assert_eq!(branches_full, branches_wo_exp);
}

// FIXME: The test below fails, however this is very likely due to the data itself that is provided by LLVM.
// For now, the test is left disabled and this should be reported upstream :)
#[cfg(any())]
#[test]
fn parse_1_file_branch_count_vs_function_branch_count() {
    let json_data = read_json_report(WO_EXPANSIONS_EXPORT_V2_0_1_PATH);
    let report: CoverageReport = serde_json::from_str(&json_data).unwrap();

    for file in report.data[0].files.iter() {
        dbg!("---------------------------------");
        let summary_cnt = file.summary.branches.count;
        let raw_cnt = file.branches.len() * 2;

        dbg!(&file.branches);
        //dbg!(file.expansions.as_ref().unwrap());
        dbg!(file.branches.len());
        dbg!(summary_cnt);
        dbg!(raw_cnt);
        dbg!(file.filename);
        assert_eq!(summary_cnt as usize, raw_cnt);
    }
}
