use std::fmt;

/// Errors returned by Matrical's checked public API.
///
/// Callers should normally match structural geometry, construction, indexing,
/// context, validation, and interchange variants directly. `Regular`, `Custom`,
/// and `ShouldNotOccur` are retained from the historical 0.1.0 prototype and are
/// not the preferred basis for new APIs.
#[derive(Debug, PartialEq, Eq)]
pub enum MatricalError {
    /// Historical operation error category retained for prototype compatibility.
    Regular(MatricalErrorType),
    /// Historical free-form error retained for prototype compatibility.
    Custom(String),
    /// A supplied value or policy failed its typed validation contract.
    InvalidValue,
    /// Required typed Gear context was absent.
    InvalidContext,
    /// An internal invariant believed unreachable was violated.
    ShouldNotOccur,
    /// An Index was outside the Matrix or Lens-local shape being accessed.
    IndexOutOfBounds,
    /// `rows * columns` cannot be represented by `usize`.
    ShapeElementCountOverflow { rows: usize, columns: usize },
    /// Row-major construction received a value count different from Shape::len.
    RowMajorLengthMismatch { expected: usize, actual: usize },
    /// A dense snapshot used a schema version this Matrical build does not support.
    UnsupportedSnapshotVersion { found: u32, supported: u32 },
    /// Snapshot dimensions cannot be represented by the receiving platform's `usize`.
    SnapshotDimensionOutOfRange { rows: u64, columns: u64 },
    /// A Region start boundary was greater than its corresponding end boundary.
    RegionReversed {
        start_row: usize,
        end_row: usize,
        start_column: usize,
        end_column: usize,
    },
    /// A Region boundary exceeded the Shape it was validated against.
    RegionOutOfBounds {
        shape_rows: usize,
        shape_columns: usize,
        start_row: usize,
        end_row: usize,
        start_column: usize,
        end_column: usize,
    },
}

#[allow(dead_code)]
pub enum AtomicBoolError {
    MutexPoisoned,
    IndexOutOfBounds,
    MissingOperand,
}

/// Historical prototype error categories retained for structural compatibility.
#[derive(Debug, PartialEq, Eq)]
pub enum MatricalErrorType {
    IncorrectDimensions,
    IncorrectFormat,
}

impl fmt::Display for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatricalError::Regular(err) => write!(f, "legacy operation error: {}", err.as_str()),
            MatricalError::Custom(err) => write!(f, "custom Matrical error: {err}"),
            MatricalError::ShouldNotOccur => {
                write!(f, "an internal Matrical invariant was violated")
            }
            MatricalError::InvalidValue => write!(f, "value failed validation"),
            MatricalError::InvalidContext => write!(f, "required Gear context is missing"),
            MatricalError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            MatricalError::ShapeElementCountOverflow { rows, columns } => write!(
                f,
                "Matrix shape {rows}x{columns} overflows the element-count range"
            ),
            MatricalError::RowMajorLengthMismatch { expected, actual } => write!(
                f,
                "Row-major data length mismatch: expected {expected} elements, got {actual}"
            ),
            MatricalError::UnsupportedSnapshotVersion { found, supported } => write!(
                f,
                "Unsupported dense snapshot version {found}; this Matrical build supports version {supported}"
            ),
            MatricalError::SnapshotDimensionOutOfRange { rows, columns } => write!(
                f,
                "Dense snapshot dimensions {rows}x{columns} cannot fit this platform's Matrix dimensions"
            ),
            MatricalError::RegionReversed {
                start_row,
                end_row,
                start_column,
                end_column,
            } => write!(
                f,
                "Region bounds are reversed: rows {start_row}..{end_row}, columns {start_column}..{end_column}"
            ),
            MatricalError::RegionOutOfBounds {
                shape_rows,
                shape_columns,
                start_row,
                end_row,
                start_column,
                end_column,
            } => write!(
                f,
                "Region rows {start_row}..{end_row}, columns {start_column}..{end_column} exceed shape {shape_rows}x{shape_columns}"
            ),
        }
    }
}

impl std::error::Error for MatricalError {}

impl MatricalErrorType {
    fn as_str(&self) -> &str {
        match *self {
            MatricalErrorType::IncorrectDimensions => "IncorrectDimensions",
            MatricalErrorType::IncorrectFormat => "IncorrectFormat",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::{MatricalError, MatricalErrorType};

    #[test]
    fn debug_formatting_identifies_regular_error_types() {
        let cases = [
            (
                MatricalErrorType::IncorrectDimensions,
                "IncorrectDimensions",
            ),
            (MatricalErrorType::IncorrectFormat, "IncorrectFormat"),
        ];

        for (error_type, expected) in cases {
            let rendered = format!("{:?}", MatricalError::Regular(error_type));

            assert!(rendered.contains("Regular"));
            assert!(rendered.contains(expected));
        }
    }

    #[test]
    fn debug_formatting_identifies_non_regular_variants() {
        let cases = [
            (
                MatricalError::Custom("r1c custom context".to_string()),
                "Custom",
                Some("r1c custom context"),
            ),
            (MatricalError::InvalidValue, "InvalidValue", None),
            (MatricalError::InvalidContext, "InvalidContext", None),
            (MatricalError::ShouldNotOccur, "ShouldNotOccur", None),
            (MatricalError::IndexOutOfBounds, "IndexOutOfBounds", None),
        ];

        for (error, expected_variant, expected_context) in cases {
            let rendered = format!("{:?}", error);

            assert!(rendered.contains(expected_variant));

            if let Some(expected_context) = expected_context {
                assert!(rendered.contains(expected_context));
            }
        }
    }

    #[test]
    fn core_errors_are_structurally_inspectable() {
        assert!(matches!(
            MatricalError::ShapeElementCountOverflow {
                rows: usize::MAX,
                columns: 2,
            },
            MatricalError::ShapeElementCountOverflow { .. }
        ));
        assert_eq!(
            MatricalError::RowMajorLengthMismatch {
                expected: 4,
                actual: 3,
            },
            MatricalError::RowMajorLengthMismatch {
                expected: 4,
                actual: 3,
            }
        );
        assert_eq!(
            MatricalError::UnsupportedSnapshotVersion {
                found: 2,
                supported: 1,
            },
            MatricalError::UnsupportedSnapshotVersion {
                found: 2,
                supported: 1,
            }
        );
        assert!(matches!(
            MatricalError::SnapshotDimensionOutOfRange {
                rows: u64::MAX,
                columns: 1,
            },
            MatricalError::SnapshotDimensionOutOfRange { .. }
        ));
    }

    #[test]
    fn caller_facing_display_is_specific_without_erasing_variant_identity() {
        let cases = [
            (MatricalError::InvalidValue, "failed validation"),
            (MatricalError::InvalidContext, "context is missing"),
            (MatricalError::IndexOutOfBounds, "out of bounds"),
            (MatricalError::ShouldNotOccur, "internal Matrical invariant"),
        ];

        for (error, expected) in cases {
            assert!(error.to_string().contains(expected));
        }

        assert!(MatricalError::UnsupportedSnapshotVersion {
            found: 2,
            supported: 1,
        }
        .to_string()
        .contains("version 2"));
        assert!(MatricalError::SnapshotDimensionOutOfRange {
            rows: u64::MAX,
            columns: 1,
        }
        .to_string()
        .contains("cannot fit"));
    }
}
