# llvm-cov-json-rs
This library can parse `llvm-cov` reports exported as JSON.
Such JSON exports are typically created by using the following command:
```bash
# Dump JSON export to stdout.
llvm-cov export --format=text --instr-profile <profile-data>
```

For more details on LLVM's source code-based coverage, see [here](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html).

## Example

```rust
use std::fs;
use llvm_cov_json::{CoverageReport};

let json_data = fs::read_to_string("coverage-report.json").unwrap()
let report: CoverageReport = serde_json::from_str(&json_data).unwrap();

/// Get the total count of branches from the summary.
let summary_branch_count = report.data[0].summary.branches.count;
println!("summary_branch_count: {}", summary_branch_count);
```