use crate::{MatricalError, Tag};
use ndarray::{s, Array2, ArrayView2, ArrayViewMut2};
use std::any::Any;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

/// A validated two-dimensional matrix shape.
///
/// Zero-sized dimensions are valid. Construction proves that the total element
/// count can be represented by `usize`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shape {
    rows: usize,
    columns: usize,
    len: usize,
}

impl Shape {
    /// Creates a checked shape.
    pub fn new(rows: usize, columns: usize) -> Result<Self, MatricalError> {
        let len = rows
            .checked_mul(columns)
            .ok_or(MatricalError::ShapeElementCountOverflow { rows, columns })?;

        Ok(Self {
            rows,
            columns,
            len,
        })
    }

    pub const fn rows(self) -> usize {
        self.rows
    }

    pub const fn columns(self) -> usize {
        self.columns
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

/// A row/column coordinate. Matrix access validates it against the Matrix shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Index {
    row: usize,
    column: usize,
}

impl Index {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub const fn row(self) -> usize {
        self.row
    }

    pub const fn column(self) -> usize {
        self.column
    }
}

/// A checked half-open rectangular region: `[start_row, end_row)` by
/// `[start_column, end_column)`.
///
/// Empty regions are valid. Reversed or out-of-bounds ranges return typed
/// errors rather than panicking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Region {
    start_row: usize,
    end_row: usize,
    start_column: usize,
    end_column: usize,
}

impl Region {
    /// Validates half-open row and column ranges against `shape`.
    pub fn new(
        shape: Shape,
        rows: Range<usize>,
        columns: Range<usize>,
    ) -> Result<Self, MatricalError> {
        Self::from_bounds(
            shape,
            rows.start,
            rows.end,
            columns.start,
            columns.end,
        )
    }

    fn from_bounds(
        shape: Shape,
        start_row: usize,
        end_row: usize,
        start_column: usize,
        end_column: usize,
    ) -> Result<Self, MatricalError> {
        if start_row > end_row || start_column > end_column {
            return Err(MatricalError::RegionReversed {
                start_row,
                end_row,
                start_column,
                end_column,
            });
        }

        if start_row > shape.rows()
            || end_row > shape.rows()
            || start_column > shape.columns()
            || end_column > shape.columns()
        {
            return Err(MatricalError::RegionOutOfBounds {
                shape_rows: shape.rows(),
                shape_columns: shape.columns(),
                start_row,
                end_row,
                start_column,
                end_column,
            });
        }

        Ok(Self {
            start_row,
            end_row,
            start_column,
            end_column,
        })
    }

    pub const fn start_row(self) -> usize {
        self.start_row
    }

    pub const fn end_row(self) -> usize {
        self.end_row
    }

    pub const fn start_column(self) -> usize {
        self.start_column
    }

    pub const fn end_column(self) -> usize {
        self.end_column
    }

    pub const fn rows(self) -> usize {
        self.end_row - self.start_row
    }

    pub const fn columns(self) -> usize {
        self.end_column - self.start_column
    }

    pub const fn is_empty(self) -> bool {
        self.rows() == 0 || self.columns() == 0
    }
}

/// An owned, validated, two-dimensional dense matrix.
///
/// Construction, iteration, and owned conversion all use logical row-major
/// order. The underlying `ndarray::Array2<T>` is intentionally private so
/// callers cannot reshape storage behind Matrical's checked shape contract.
///
/// # Example
///
/// ```
/// use matrical::{Index, Matrix, Shape};
///
/// fn main() -> Result<(), matrical::MatricalError> {
///     let shape = Shape::new(2, 2)?;
///     let matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4])?;
///
///     assert_eq!(matrix.get(Index::new(1, 0))?, &3);
///     assert_eq!(matrix.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
///     Ok(())
/// }
/// ```
pub struct Matrix<T> {
    shape: Shape,
    values: Array2<T>,
}

impl<T> Matrix<T> {
    /// Builds a Matrix from exactly `shape.len()` values in row-major order.
    pub fn from_row_major(shape: Shape, values: Vec<T>) -> Result<Self, MatricalError> {
        if values.len() != shape.len() {
            return Err(MatricalError::RowMajorLengthMismatch {
                expected: shape.len(),
                actual: values.len(),
            });
        }

        let values = Array2::from_shape_vec((shape.rows(), shape.columns()), values)
            .map_err(|_| MatricalError::ShouldNotOccur)?;

        Ok(Self { shape, values })
    }

    pub const fn shape(&self) -> Shape {
        self.shape
    }

    pub const fn rows(&self) -> usize {
        self.shape.rows()
    }

    pub const fn columns(&self) -> usize {
        self.shape.columns()
    }

    pub const fn len(&self) -> usize {
        self.shape.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    /// Returns a checked immutable element reference.
    pub fn get(&self, index: Index) -> Result<&T, MatricalError> {
        if index.row() >= self.rows() || index.column() >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        self.values
            .get((index.row(), index.column()))
            .ok_or(MatricalError::IndexOutOfBounds)
    }

    /// Returns a checked mutable element reference.
    pub fn get_mut(&mut self, index: Index) -> Result<&mut T, MatricalError> {
        if index.row() >= self.rows() || index.column() >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        self.values
            .get_mut((index.row(), index.column()))
            .ok_or(MatricalError::IndexOutOfBounds)
    }

    /// Iterates over values in deterministic logical row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    /// Mutably iterates over values in deterministic logical row-major order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    /// Establishes a checked half-open Region against this Matrix's Shape.
    pub fn region(
        &self,
        rows: Range<usize>,
        columns: Range<usize>,
    ) -> Result<Region, MatricalError> {
        Region::new(self.shape, rows, columns)
    }

    /// Revalidates a Region against this Matrix's Shape.
    pub fn validate_region(&self, region: Region) -> Result<Region, MatricalError> {
        Region::from_bounds(
            self.shape,
            region.start_row(),
            region.end_row(),
            region.start_column(),
            region.end_column(),
        )
    }

    pub(crate) fn checked_region_view(
        &self,
        region: Region,
    ) -> Result<ArrayView2<'_, T>, MatricalError> {
        let region = self.validate_region(region)?;
        Ok(self.values.slice(s![
            region.start_row()..region.end_row(),
            region.start_column()..region.end_column()
        ]))
    }

    pub(crate) fn checked_region_view_mut(
        &mut self,
        region: Region,
    ) -> Result<ArrayViewMut2<'_, T>, MatricalError> {
        let region = self.validate_region(region)?;
        Ok(self.values.slice_mut(s![
            region.start_row()..region.end_row(),
            region.start_column()..region.end_column()
        ]))
    }

    /// Consumes the Matrix and returns values in construction/iteration order.
    pub fn into_row_major(self) -> Vec<T> {
        self.values.into_raw_vec()
    }
}

/// Historical operation scaffolding retained only so unrelated prototype
/// modules continue to compile. It is not Matrix storage and does not define the
/// new Matrix shape or ownership invariant.
pub struct MatrixContext {
    dimensions: Option<(usize, usize)>,
    attributes: Vec<Tag>,
    functors: HashMap<usize, Arc<Box<dyn Fn(dyn Any + Send + Sync) -> ()>>>,
}

#[cfg(test)]
mod tests {
    use super::{Index, Matrix, Region, Shape};
    use crate::MatricalError;

    #[test]
    fn shape_accepts_ordinary_dimensions() {
        let shape = Shape::new(3, 4).expect("ordinary shape should be valid");
        assert_eq!(shape.rows(), 3);
        assert_eq!(shape.columns(), 4);
        assert_eq!(shape.len(), 12);
        assert!(!shape.is_empty());
    }

    #[test]
    fn shape_accepts_zero_sized_dimensions() {
        for (rows, columns) in [(0, 0), (0, 5), (5, 0)] {
            let shape = Shape::new(rows, columns).expect("zero-sized shape should be valid");
            assert_eq!(shape.len(), 0);
            assert!(shape.is_empty());
        }
    }

    #[test]
    fn shape_rejects_element_count_overflow() {
        assert_eq!(
            Shape::new(usize::MAX, 2),
            Err(MatricalError::ShapeElementCountOverflow {
                rows: usize::MAX,
                columns: 2,
            })
        );
    }

    #[test]
    fn row_major_construction_exposes_geometry_and_access() {
        let shape = Shape::new(2, 3).unwrap();
        let matrix = Matrix::from_row_major(shape, vec![0, 1, 2, 3, 4, 5]).unwrap();

        assert_eq!(matrix.shape(), shape);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.columns(), 3);
        assert_eq!(matrix.len(), 6);
        assert!(!matrix.is_empty());
        assert_eq!(*matrix.get(Index::new(0, 0)).unwrap(), 0);
        assert_eq!(*matrix.get(Index::new(1, 2)).unwrap(), 5);
    }

    #[test]
    fn row_major_construction_rejects_short_input() {
        let shape = Shape::new(2, 2).unwrap();
        assert!(matches!(
            Matrix::from_row_major(shape, vec![1, 2, 3]),
            Err(MatricalError::RowMajorLengthMismatch {
                expected: 4,
                actual: 3,
            })
        ));
    }

    #[test]
    fn row_major_construction_rejects_long_input() {
        let shape = Shape::new(2, 2).unwrap();
        assert!(matches!(
            Matrix::from_row_major(shape, vec![1, 2, 3, 4, 5]),
            Err(MatricalError::RowMajorLengthMismatch {
                expected: 4,
                actual: 5,
            })
        ));
    }

    #[test]
    fn zero_sized_matrix_construction_succeeds() {
        for shape in [
            Shape::new(0, 0).unwrap(),
            Shape::new(0, 4).unwrap(),
            Shape::new(4, 0).unwrap(),
        ] {
            let matrix = Matrix::<i32>::from_row_major(shape, vec![]).unwrap();
            assert_eq!(matrix.shape(), shape);
            assert!(matrix.is_empty());
            assert_eq!(matrix.len(), 0);
        }
    }

    #[test]
    fn invalid_index_access_fails_safely() {
        let shape = Shape::new(2, 3).unwrap();
        let matrix = Matrix::from_row_major(shape, vec![0, 1, 2, 3, 4, 5]).unwrap();

        assert!(matches!(
            matrix.get(Index::new(2, 0)),
            Err(MatricalError::IndexOutOfBounds)
        ));
        assert!(matches!(
            matrix.get(Index::new(0, 3)),
            Err(MatricalError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn empty_matrix_access_fails_safely() {
        let shape = Shape::new(0, 4).unwrap();
        let matrix = Matrix::<i32>::from_row_major(shape, vec![]).unwrap();

        assert!(matches!(
            matrix.get(Index::new(0, 0)),
            Err(MatricalError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn mutable_access_changes_only_the_target_and_rejects_invalid_input() {
        let shape = Shape::new(2, 2).unwrap();
        let mut matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4]).unwrap();

        *matrix.get_mut(Index::new(1, 0)).unwrap() = 30;
        assert_eq!(matrix.iter().copied().collect::<Vec<_>>(), vec![1, 2, 30, 4]);
        assert!(matches!(
            matrix.get_mut(Index::new(2, 0)),
            Err(MatricalError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn iteration_is_logical_row_major() {
        let shape = Shape::new(2, 3).unwrap();
        let matrix = Matrix::from_row_major(shape, vec![0, 1, 2, 3, 4, 5]).unwrap();

        assert_eq!(matrix.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn mutable_iteration_preserves_logical_row_major_order() {
        let shape = Shape::new(2, 3).unwrap();
        let mut matrix = Matrix::from_row_major(shape, vec![0, 0, 0, 0, 0, 0]).unwrap();

        for (value, replacement) in matrix.iter_mut().zip(10..16) {
            *value = replacement;
        }

        assert_eq!(matrix.into_row_major(), vec![10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn valid_and_full_regions_are_accepted() {
        let shape = Shape::new(3, 4).unwrap();
        let region = Region::new(shape, 1..3, 1..4).unwrap();
        assert_eq!(region.rows(), 2);
        assert_eq!(region.columns(), 3);
        assert!(!region.is_empty());

        let full = Region::new(shape, 0..3, 0..4).unwrap();
        assert_eq!(full.rows(), 3);
        assert_eq!(full.columns(), 4);
    }

    #[test]
    fn region_end_boundary_may_equal_shape() {
        let shape = Shape::new(2, 3).unwrap();
        assert!(Region::new(shape, 2..2, 3..3).is_ok());
    }

    #[test]
    fn reversed_region_bounds_are_rejected() {
        let shape = Shape::new(3, 4).unwrap();
        let reversed_rows = std::ops::Range { start: 2, end: 1 };
        let reversed_columns = std::ops::Range { start: 3, end: 2 };

        assert!(matches!(
            Region::new(shape, reversed_rows, 0..1),
            Err(MatricalError::RegionReversed { .. })
        ));
        assert!(matches!(
            Region::new(shape, 0..1, reversed_columns),
            Err(MatricalError::RegionReversed { .. })
        ));
    }

    #[test]
    fn out_of_bounds_regions_are_rejected() {
        let shape = Shape::new(3, 4).unwrap();
        assert!(matches!(
            Region::new(shape, 0..4, 0..1),
            Err(MatricalError::RegionOutOfBounds { .. })
        ));
        assert!(matches!(
            Region::new(shape, 0..1, 0..5),
            Err(MatricalError::RegionOutOfBounds { .. })
        ));
    }

    #[test]
    fn empty_regions_are_valid() {
        let shape = Shape::new(3, 4).unwrap();
        assert!(Region::new(shape, 1..1, 0..4).unwrap().is_empty());
        assert!(Region::new(shape, 0..3, 2..2).unwrap().is_empty());
    }

    #[test]
    fn matrix_establishes_and_revalidates_regions() {
        let shape = Shape::new(2, 2).unwrap();
        let matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4]).unwrap();
        let region = matrix.region(0..2, 1..2).unwrap();
        assert_eq!(matrix.validate_region(region), Ok(region));

        let larger = Region::new(Shape::new(3, 3).unwrap(), 0..3, 0..3).unwrap();
        assert!(matches!(
            matrix.validate_region(larger),
            Err(MatricalError::RegionOutOfBounds { .. })
        ));
    }

    #[test]
    fn owned_row_major_conversion_round_trips_construction_order() {
        let shape = Shape::new(2, 3).unwrap();
        let values = vec![7, 8, 9, 10, 11, 12];
        let matrix = Matrix::from_row_major(shape, values.clone()).unwrap();

        assert_eq!(matrix.into_row_major(), values);
    }
}
