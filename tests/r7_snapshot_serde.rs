#![cfg(feature = "serde")]

use matrical::{MatricalError, Matrix, MatrixSnapshot, Shape, DENSE_SNAPSHOT_VERSION};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("fixtures/r7_dense_snapshot_v1.json");

#[test]
fn committed_v1_fixture_deserializes_and_reconstructs() {
    let snapshot: MatrixSnapshot<i64> = serde_json::from_str(FIXTURE).unwrap();

    assert_eq!(snapshot.version(), DENSE_SNAPSHOT_VERSION);
    assert_eq!(snapshot.rows(), 2);
    assert_eq!(snapshot.columns(), 3);
    assert_eq!(snapshot.row_major(), &[1, 2, 3, 4, 5, 6]);

    let matrix = snapshot.into_matrix().unwrap();
    assert_eq!(matrix.shape(), Shape::new(2, 3).unwrap());
    assert_eq!(matrix.into_row_major(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn matching_snapshot_serializes_semantically_to_fixture() {
    let shape = Shape::new(2, 3).unwrap();
    let snapshot = Matrix::from_row_major(shape, vec![1_i64, 2, 3, 4, 5, 6])
        .unwrap()
        .into_snapshot();

    let actual: Value = serde_json::to_value(snapshot).unwrap();
    let expected: Value = serde_json::from_str(FIXTURE).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn unsupported_version_fails_closed_during_reconstruction() {
    let snapshot: MatrixSnapshot<i64> = serde_json::from_value(json!({
        "version": 2,
        "rows": 1,
        "columns": 1,
        "row_major": [7]
    }))
    .unwrap();

    assert!(matches!(
        snapshot.into_matrix(),
        Err(MatricalError::UnsupportedSnapshotVersion {
            found: 2,
            supported: DENSE_SNAPSHOT_VERSION,
        })
    ));
}

#[test]
fn malformed_row_major_length_uses_existing_structural_error() {
    let snapshot: MatrixSnapshot<i64> = serde_json::from_value(json!({
        "version": 1,
        "rows": 2,
        "columns": 3,
        "row_major": [1, 2]
    }))
    .unwrap();

    assert!(matches!(
        snapshot.into_matrix(),
        Err(MatricalError::RowMajorLengthMismatch {
            expected: 6,
            actual: 2,
        })
    ));
}

#[test]
fn oversized_or_overflowing_dimensions_fail_without_truncation() {
    let snapshot: MatrixSnapshot<i64> = serde_json::from_value(json!({
        "version": 1,
        "rows": u64::MAX,
        "columns": 2,
        "row_major": []
    }))
    .unwrap();

    let error = match snapshot.into_matrix() {
        Err(error) => error,
        Ok(_) => panic!("oversized snapshot must not reconstruct"),
    };

    #[cfg(target_pointer_width = "64")]
    assert!(matches!(
        error,
        MatricalError::ShapeElementCountOverflow {
            rows: usize::MAX,
            columns: 2,
        }
    ));

    #[cfg(target_pointer_width = "32")]
    assert!(matches!(
        error,
        MatricalError::SnapshotDimensionOutOfRange {
            rows: u64::MAX,
            columns: 2,
        }
    ));
}

#[test]
fn v1_deserialization_denies_unknown_fields() {
    let result = serde_json::from_value::<MatrixSnapshot<i64>>(json!({
        "version": 1,
        "rows": 1,
        "columns": 1,
        "row_major": [7],
        "future_semantics": true
    }));

    assert!(result.is_err());
}

#[test]
fn zero_dimensions_roundtrip_through_json() {
    for (rows, columns) in [(0_u64, 0_u64), (0, 5), (5, 0)] {
        let snapshot: MatrixSnapshot<i64> = serde_json::from_value(json!({
            "version": 1,
            "rows": rows,
            "columns": columns,
            "row_major": []
        }))
        .unwrap();

        let matrix = snapshot.into_matrix().unwrap();
        assert_eq!(matrix.rows() as u64, rows);
        assert_eq!(matrix.columns() as u64, columns);
        assert!(matrix.is_empty());
    }
}
