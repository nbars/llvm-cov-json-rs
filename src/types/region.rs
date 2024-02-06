use serde::{Deserialize, Deserializer, de::Error};


/// The kind a code region can belong to.
/// Take from the [LLVM source code](https://github.com/llvm/llvm-project/blob/bd611264993f64decbce178d460caf1d1cb05f59/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L220).
#[derive(Debug, PartialEq)]
pub enum RegionKind {
    /// A CodeRegion associates some code with a counter
    Code,
    /// An Expansion region represents a file expansion region that associates
    /// a source range with the expansion of a virtual source file, such as
    /// for a macro instantiation or #include file.
    Expansion,
    /// A Skipped region represents a source range with code that was skipped
    /// by a preprocessor or similar means.
    Skipped,
    /// A Gap region is like a CodeRegion, but its count is only set as the
    /// line execution count when its the only region in the line.
    Gap,
    /// A Branch region represents leaf-level boolean expressions and is
    /// associated with two counters, each representing the number of times the
    /// expression evaluates to true or false.
    Branch
}

impl TryFrom<u64> for RegionKind {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let ret = match value {
            _ if value == RegionKind::Code as u64 => RegionKind::Code,
            _ if value == RegionKind::Expansion as u64 => RegionKind::Expansion,
            _ if value == RegionKind::Skipped as u64 => RegionKind::Skipped,
            _ if value == RegionKind::Gap as u64 => RegionKind::Gap,
            _ if value == RegionKind::Branch as u64 => RegionKind::Branch,
            _ => return Err(format!("Unknown region kind {}", value)),
        };

        Ok(ret)
    }
}

#[derive(Debug, PartialEq)]
pub struct Region {
    /// The source code line this region starts.
    line_start: u64,
    /// The column of `line_start` where this region starts.
    column_start: u64,
    /// The source code line this region ends.
    line_end: u64,
    /// The column of `line_end` where this region ends.
    column_end: u64,
    /// Number of times this region was executed.
    execution_count: u64,
    /// A unique id that identifies the file this region belongs to.
    file_id: u64,
    /// The file id from which this region comes from if this is an [`RegionKind::Expansion`].
    /// I.e., the header where the a macro was defined or the include points.
    expanded_file_id: u64,
    /// The kind of region.
    kind: RegionKind,
}


#[derive(Deserialize)]
struct RegionTuple(u64, u64, u64, u64, u64, u64, u64, u64);

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let tuple = RegionTuple::deserialize(deserializer)?;
        let kind = match RegionKind::try_from(tuple.7) {
            Ok(kind) => kind,
            Err(err) => return Err(Error::custom(err)),
        };

        Ok(Region {
            line_start: tuple.0,
            column_start: tuple.1,
            line_end: tuple.2,
            column_end: tuple.3,
            execution_count: tuple.4,
            file_id: tuple.5,
            expanded_file_id: tuple.6,
            kind,
        })
    }
}
