//! Versioned, inert dense Matrix snapshots for interchange boundaries.
//!
//! `MatrixSnapshot<T>` owns logical row-major values and stable wire dimensions.
//! It contains no ndarray type, live storage handle, filesystem/database/network
//! authority, or execution capability. Reconstruction always revalidates the
//! snapshot through Matrical's existing `Shape` and `Matrix` invariants.

use crate::{MatricalError, Matrix, Shape};

/// Schema version emitted and accepted by the dense snapshot v1 API.
pub const DENSE_SNAPSHOT_VERSION: u32 = 1;

/// An inert, versioned dense Matrix representation suitable for interchange.
///
/// Fields are private so ordinary callers cannot independently mutate schema
/// version or shape metadata. With the optional `serde` feature, external data
/// may deserialize into a snapshot, but [`MatrixSnapshot::into_matrix`] always
/// validates it before creating a live [`Matrix`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct MatrixSnapshot<T> {
    version: u32,
    rows: u64,
    columns: u64,
    row_major: Vec<T>,
}

impl<T> MatrixSnapshot<T> {
    /// Returns the snapshot schema version.
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the row count in the representation-stable interchange type.
    pub const fn rows(&self) -> u64 {
        self.rows
    }

    /// Returns the column count in the representation-stable interchange type.
    pub const fn columns(&self) -> u64 {
        self.columns
    }

    /// Returns the number of stored row-major values.
    pub fn len(&self) -> usize {
        self.row_major.len()
    }

    /// Returns whether the snapshot stores zero values.
    pub fn is_empty(&self) -> bool {
        self.row_major.is_empty()
    }

    /// Borrows the stored values in logical row-major order.
    pub fn row_major(&self) -> &[T] {
        &self.row_major
    }

    /// Consumes the snapshot and returns its stored row-major values.
    pub fn into_row_major(self) -> Vec<T> {
        self.row_major
    }

    /// Validates this snapshot and constructs a fresh owned dense Matrix.
    ///
    /// Reconstruction fails closed for unsupported schema versions, dimensions
    /// that cannot fit the receiving platform, shape element-count overflow, or
    /// row-major length mismatch.
    pub fn into_matrix(self) -> Result<Matrix<T>, MatricalError> {
        if self.version != DENSE_SNAPSHOT_VERSION {
            return Err(MatricalError::UnsupportedSnapshotVersion {
                found: self.version,
                supported: DENSE_SNAPSHOT_VERSION,
            });
        }

        let shape = checked_shape(self.rows, self.columns)?;
        Matrix::from_row_major(shape, self.row_major)
    }
}

impl<T: Clone> Matrix<T> {
    /// Clones this Matrix's values into an inert dense snapshot.
    ///
    /// This is an O(n) value clone and therefore requires `T: Clone`.
    pub fn snapshot(&self) -> MatrixSnapshot<T> {
        MatrixSnapshot {
            version: DENSE_SNAPSHOT_VERSION,
            rows: self.rows() as u64,
            columns: self.columns() as u64,
            row_major: self.iter().cloned().collect(),
        }
    }
}

impl<T> Matrix<T> {
    /// Consumes this Matrix and transfers its owned values into a snapshot.
    ///
    /// No `T: Clone` bound is required because element ownership is transferred.
    pub fn into_snapshot(self) -> MatrixSnapshot<T> {
        let rows = self.rows() as u64;
        let columns = self.columns() as u64;
        let row_major = self.into_row_major();

        MatrixSnapshot {
            version: DENSE_SNAPSHOT_VERSION,
            rows,
            columns,
            row_major,
        }
    }
}

impl<T> TryFrom<MatrixSnapshot<T>> for Matrix<T> {
    type Error = MatricalError;

    fn try_from(snapshot: MatrixSnapshot<T>) -> Result<Self, Self::Error> {
        snapshot.into_matrix()
    }
}

fn checked_shape(rows: u64, columns: u64) -> Result<Shape, MatricalError> {
    let rows_usize = usize::try_from(rows)
        .map_err(|_| MatricalError::SnapshotDimensionOutOfRange { rows, columns })?;
    let columns_usize = usize::try_from(columns)
        .map_err(|_| MatricalError::SnapshotDimensionOutOfRange { rows, columns })?;

    Shape::new(rows_usize, columns_usize)
}

#[cfg(test)]
mod tests {
    use super::checked_shape;
    use crate::MatricalError;

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn checked_shape_rejects_dimensions_that_do_not_fit_usize() {
        let rows = u64::from(u32::MAX) + 1;
        assert_eq!(
            checked_shape(rows, 1),
            Err(MatricalError::SnapshotDimensionOutOfRange { rows, columns: 1 })
        );
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn checked_shape_reuses_shape_overflow_validation_on_64_bit_targets() {
        assert!(matches!(
            checked_shape(u64::MAX, 2),
            Err(MatricalError::ShapeElementCountOverflow { .. })
        ));
    }
}
