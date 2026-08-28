use crate::{Index, MatricalError, Matrix, Region, Shape};

/// An immutable, zero-copy rectangular borrowing view over a [`Matrix`].
///
/// A `Lens` borrows its parent Matrix for the full Lens lifetime, so it cannot
/// outlive the Matrix. The selected [`Region`] is half-open in parent
/// coordinates, while [`Index`] values passed to [`Lens::get`] are local to the
/// selected rectangle: local `(0, 0)` is the Region's top-left element.
///
/// Construction, checked access, and iteration do not allocate. Iteration is
/// deterministic logical row-major order over the selected rectangle; no
/// physical-contiguity guarantee is made. [`Lens::to_row_major`] is the explicit
/// allocating conversion and requires `T: Clone`.
///
/// # Example
///
/// ```
/// use matrical::{Index, Matrix, Region, Shape};
///
/// fn main() -> Result<(), matrical::MatricalError> {
///     let shape = Shape::new(3, 4)?;
///     let matrix = Matrix::from_row_major(shape, (0..12).collect())?;
///     let region = Region::new(shape, 1..3, 1..4)?;
///     let lens = matrix.lens(region)?;
///
///     assert_eq!(lens.shape(), Shape::new(2, 3)?);
///     assert_eq!(lens.get(Index::new(0, 0))?, &5);
///     assert_eq!(lens.iter().copied().collect::<Vec<_>>(), vec![5, 6, 7, 9, 10, 11]);
///     Ok(())
/// }
/// ```
///
/// A Lens cannot escape the Matrix it borrows:
///
/// ```compile_fail
/// use matrical::{Lens, MatricalError, Matrix, Region, Shape};
///
/// fn escaped<'a>() -> Result<Lens<'a, i32>, MatricalError> {
///     let shape = Shape::new(1, 1)?;
///     let matrix = Matrix::from_row_major(shape, vec![1])?;
///     let region = Region::new(shape, 0..1, 0..1)?;
///     matrix.lens(region)
/// }
/// ```
pub struct Lens<'a, T> {
    matrix: &'a Matrix<T>,
    region: Region,
    shape: Shape,
}

impl<'a, T> Lens<'a, T> {
    fn new(matrix: &'a Matrix<T>, region: Region) -> Result<Self, MatricalError> {
        let region = matrix.validate_region(region)?;
        let shape = Shape::new(region.rows(), region.columns())?;
        Ok(Self {
            matrix,
            region,
            shape,
        })
    }

    /// Returns the selected half-open Region in parent Matrix coordinates.
    pub const fn region(&self) -> Region {
        self.region
    }

    /// Returns the Lens-local Shape.
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

    /// Returns a checked element reference using Lens-local coordinates.
    pub fn get(&self, index: Index) -> Result<&T, MatricalError> {
        let parent = self.parent_index(index)?;
        self.matrix.get(parent)
    }

    /// Iterates selected values in deterministic logical row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        selected_iter(self.matrix.iter(), self.matrix.columns(), self.region)
    }

    /// Clones selected values into an owned row-major vector.
    ///
    /// This is the Lens operation that intentionally allocates.
    pub fn to_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter().cloned().collect()
    }

    fn parent_index(&self, index: Index) -> Result<Index, MatricalError> {
        if index.row() >= self.rows() || index.column() >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        Ok(Index::new(
            self.region.start_row() + index.row(),
            self.region.start_column() + index.column(),
        ))
    }
}

/// A mutable, zero-copy rectangular borrowing view over a [`Matrix`].
///
/// `LensMut` holds an exclusive mutable borrow of the parent Matrix for its
/// lifetime. Safe Rust therefore rejects a second mutable Lens created through
/// the same Matrix while the first remains live, regardless of whether the
/// Regions would be disjoint. R3 deliberately uses this conservative aliasing
/// contract rather than unsafe disjoint-splitting machinery.
///
/// Local indexing, logical row-major iteration, empty-view behavior, and the
/// explicit allocating conversion match [`Lens`].
///
/// Simultaneous mutable Lenses through one Matrix are rejected by the borrow
/// checker:
///
/// ```compile_fail
/// use matrical::{MatricalError, Matrix, Region, Shape};
///
/// fn main() -> Result<(), MatricalError> {
///     let shape = Shape::new(2, 2)?;
///     let mut matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4])?;
///     let first_region = Region::new(shape, 0..1, 0..2)?;
///     let second_region = Region::new(shape, 1..2, 0..2)?;
///
///     let first = matrix.lens_mut(first_region)?;
///     let second = matrix.lens_mut(second_region)?;
///     assert_eq!(first.len() + second.len(), 4);
///     Ok(())
/// }
/// ```
pub struct LensMut<'a, T> {
    matrix: &'a mut Matrix<T>,
    region: Region,
    shape: Shape,
}

impl<'a, T> LensMut<'a, T> {
    fn new(matrix: &'a mut Matrix<T>, region: Region) -> Result<Self, MatricalError> {
        let region = matrix.validate_region(region)?;
        let shape = Shape::new(region.rows(), region.columns())?;
        Ok(Self {
            matrix,
            region,
            shape,
        })
    }

    /// Returns the selected half-open Region in parent Matrix coordinates.
    pub const fn region(&self) -> Region {
        self.region
    }

    /// Returns the Lens-local Shape.
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

    /// Returns a checked immutable element reference using Lens-local coordinates.
    pub fn get(&self, index: Index) -> Result<&T, MatricalError> {
        let parent = self.parent_index(index)?;
        self.matrix.get(parent)
    }

    /// Returns a checked mutable element reference using Lens-local coordinates.
    pub fn get_mut(&mut self, index: Index) -> Result<&mut T, MatricalError> {
        let parent = self.parent_index(index)?;
        self.matrix.get_mut(parent)
    }

    /// Iterates selected values immutably in logical row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        selected_iter(self.matrix.iter(), self.matrix.columns(), self.region)
    }

    /// Iterates selected values mutably in logical row-major order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let parent_columns = self.matrix.columns();
        let region = self.region;
        selected_iter(self.matrix.iter_mut(), parent_columns, region)
    }

    /// Clones selected values into an owned row-major vector.
    ///
    /// This operation intentionally allocates.
    pub fn to_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter().cloned().collect()
    }

    fn parent_index(&self, index: Index) -> Result<Index, MatricalError> {
        if index.row() >= self.rows() || index.column() >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        Ok(Index::new(
            self.region.start_row() + index.row(),
            self.region.start_column() + index.column(),
        ))
    }
}

fn selected_iter<I, T>(
    iter: I,
    parent_columns: usize,
    region: Region,
) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
{
    iter.enumerate().filter_map(move |(offset, value)| {
        if parent_columns == 0 {
            return None;
        }

        let row = offset / parent_columns;
        let column = offset % parent_columns;
        (row >= region.start_row()
            && row < region.end_row()
            && column >= region.start_column()
            && column < region.end_column())
        .then_some(value)
    })
}

impl<T> Matrix<T> {
    /// Creates an immutable zero-copy Lens for `region`.
    ///
    /// The Region is revalidated against this Matrix even if it was originally
    /// constructed for another Shape.
    pub fn lens(&self, region: Region) -> Result<Lens<'_, T>, MatricalError> {
        Lens::new(self, region)
    }

    /// Creates a mutable zero-copy Lens for `region`.
    ///
    /// The returned LensMut exclusively borrows this Matrix until the LensMut is
    /// dropped or otherwise no longer live.
    pub fn lens_mut(&mut self, region: Region) -> Result<LensMut<'_, T>, MatricalError> {
        LensMut::new(self, region)
    }

    /// Selects one row as a `1 x columns` immutable Lens.
    ///
    /// For an `N x 0` Matrix, any in-range row is a valid empty `1 x 0` Lens.
    pub fn row(&self, row: usize) -> Result<Lens<'_, T>, MatricalError> {
        if row >= self.rows() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        let region = Region::new(self.shape(), row..row + 1, 0..self.columns())?;
        self.lens(region)
    }

    /// Selects one row as a `1 x columns` mutable Lens.
    pub fn row_mut(&mut self, row: usize) -> Result<LensMut<'_, T>, MatricalError> {
        if row >= self.rows() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        let region = Region::new(self.shape(), row..row + 1, 0..self.columns())?;
        self.lens_mut(region)
    }

    /// Selects one column as a `rows x 1` immutable Lens.
    ///
    /// For a `0 x N` Matrix, any in-range column is a valid empty `0 x 1` Lens.
    pub fn column(&self, column: usize) -> Result<Lens<'_, T>, MatricalError> {
        if column >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        let region = Region::new(self.shape(), 0..self.rows(), column..column + 1)?;
        self.lens(region)
    }

    /// Selects one column as a `rows x 1` mutable Lens.
    pub fn column_mut(&mut self, column: usize) -> Result<LensMut<'_, T>, MatricalError> {
        if column >= self.columns() {
            return Err(MatricalError::IndexOutOfBounds);
        }

        let region = Region::new(self.shape(), 0..self.rows(), column..column + 1)?;
        self.lens_mut(region)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lens, LensMut};
    use crate::{Index, MatricalError, Matrix, Region, Shape};

    fn matrix_3x4() -> Matrix<i32> {
        Matrix::from_row_major(Shape::new(3, 4).unwrap(), (0..12).collect()).unwrap()
    }

    #[test]
    fn immutable_rectangular_lens_uses_local_coordinates_and_row_major_iteration() {
        let matrix = matrix_3x4();
        let region = Region::new(matrix.shape(), 1..3, 1..4).unwrap();
        let lens = matrix.lens(region).unwrap();

        assert_eq!(lens.region(), region);
        assert_eq!(lens.shape(), Shape::new(2, 3).unwrap());
        assert_eq!(lens.rows(), 2);
        assert_eq!(lens.columns(), 3);
        assert_eq!(lens.len(), 6);
        assert!(!lens.is_empty());
        assert_eq!(*lens.get(Index::new(0, 0)).unwrap(), 5);
        assert_eq!(*lens.get(Index::new(1, 2)).unwrap(), 11);
        assert!(matches!(
            lens.get(Index::new(2, 0)),
            Err(MatricalError::IndexOutOfBounds)
        ));
        assert_eq!(
            lens.iter().copied().collect::<Vec<_>>(),
            vec![5, 6, 7, 9, 10, 11]
        );
        assert_eq!(lens.to_row_major(), vec![5, 6, 7, 9, 10, 11]);
    }

    #[test]
    fn full_one_element_and_empty_lenses_are_valid() {
        let matrix = matrix_3x4();

        let full = matrix
            .lens(Region::new(matrix.shape(), 0..3, 0..4).unwrap())
            .unwrap();
        assert_eq!(full.len(), 12);
        assert_eq!(full.iter().copied().collect::<Vec<_>>(), (0..12).collect::<Vec<_>>());

        let one = matrix
            .lens(Region::new(matrix.shape(), 2..3, 3..4).unwrap())
            .unwrap();
        assert_eq!(one.shape(), Shape::new(1, 1).unwrap());
        assert_eq!(*one.get(Index::new(0, 0)).unwrap(), 11);

        let empty = matrix
            .lens(Region::new(matrix.shape(), 1..1, 0..4).unwrap())
            .unwrap();
        assert_eq!(empty.shape(), Shape::new(0, 4).unwrap());
        assert!(empty.is_empty());
        assert_eq!(empty.iter().count(), 0);
        assert!(matches!(
            empty.get(Index::new(0, 0)),
            Err(MatricalError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn lens_revalidates_foreign_region_against_receiving_matrix() {
        let matrix = Matrix::from_row_major(Shape::new(2, 2).unwrap(), vec![1, 2, 3, 4]).unwrap();
        let foreign = Region::new(Shape::new(3, 3).unwrap(), 0..3, 0..3).unwrap();

        assert!(matches!(
            matrix.lens(foreign),
            Err(MatricalError::RegionOutOfBounds { .. })
        ));
    }

    #[test]
    fn mutable_lens_updates_only_selected_parent_values() {
        let mut matrix = matrix_3x4();
        let region = Region::new(matrix.shape(), 1..3, 1..3).unwrap();

        {
            let mut lens = matrix.lens_mut(region).unwrap();
            *lens.get_mut(Index::new(0, 0)).unwrap() = 50;
            assert_eq!(*lens.get(Index::new(0, 0)).unwrap(), 50);
            assert!(matches!(
                lens.get_mut(Index::new(2, 0)),
                Err(MatricalError::IndexOutOfBounds)
            ));
            for value in lens.iter_mut() {
                *value += 100;
            }
        }

        assert_eq!(
            matrix.into_row_major(),
            vec![0, 1, 2, 3, 4, 150, 106, 7, 8, 109, 110, 11]
        );
    }

    #[test]
    fn empty_mutable_lens_is_safe() {
        let mut matrix = matrix_3x4();
        let region = Region::new(matrix.shape(), 0..3, 2..2).unwrap();
        let mut lens = matrix.lens_mut(region).unwrap();

        assert_eq!(lens.shape(), Shape::new(3, 0).unwrap());
        assert!(lens.is_empty());
        assert_eq!(lens.iter().count(), 0);
        assert_eq!(lens.iter_mut().count(), 0);
    }

    #[test]
    fn row_selection_and_mutation_follow_empty_shape_rules() {
        let matrix = matrix_3x4();
        assert_eq!(matrix.row(0).unwrap().to_row_major(), vec![0, 1, 2, 3]);
        assert_eq!(matrix.row(2).unwrap().to_row_major(), vec![8, 9, 10, 11]);
        assert!(matches!(matrix.row(3), Err(MatricalError::IndexOutOfBounds)));

        let empty_columns = Matrix::<i32>::from_row_major(Shape::new(2, 0).unwrap(), vec![]).unwrap();
        let row = empty_columns.row(1).unwrap();
        assert_eq!(row.shape(), Shape::new(1, 0).unwrap());
        assert!(row.is_empty());

        let mut mutable = matrix_3x4();
        for value in mutable.row_mut(1).unwrap().iter_mut() {
            *value *= -1;
        }
        assert_eq!(
            mutable.into_row_major(),
            vec![0, 1, 2, 3, -4, -5, -6, -7, 8, 9, 10, 11]
        );
    }

    #[test]
    fn column_selection_and_mutation_follow_empty_shape_rules() {
        let matrix = matrix_3x4();
        assert_eq!(matrix.column(0).unwrap().to_row_major(), vec![0, 4, 8]);
        assert_eq!(matrix.column(3).unwrap().to_row_major(), vec![3, 7, 11]);
        assert!(matches!(
            matrix.column(4),
            Err(MatricalError::IndexOutOfBounds)
        ));

        let empty_rows = Matrix::<i32>::from_row_major(Shape::new(0, 3).unwrap(), vec![]).unwrap();
        let column = empty_rows.column(2).unwrap();
        assert_eq!(column.shape(), Shape::new(0, 1).unwrap());
        assert!(column.is_empty());

        let mut mutable = matrix_3x4();
        for value in mutable.column_mut(2).unwrap().iter_mut() {
            *value += 20;
        }
        assert_eq!(
            mutable.into_row_major(),
            vec![0, 1, 22, 3, 4, 5, 26, 7, 8, 9, 30, 11]
        );
    }

    #[test]
    fn zero_by_zero_has_no_valid_row_or_column() {
        let matrix = Matrix::<i32>::from_row_major(Shape::new(0, 0).unwrap(), vec![]).unwrap();
        assert!(matches!(matrix.row(0), Err(MatricalError::IndexOutOfBounds)));
        assert!(matches!(
            matrix.column(0),
            Err(MatricalError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn boundary_regions_are_exhaustively_checked_over_small_shapes() {
        let shapes = [(0, 0), (0, 3), (3, 0), (1, 1), (1, 4), (4, 1), (2, 3), (3, 4)];

        for (rows, columns) in shapes {
            let shape = Shape::new(rows, columns).unwrap();
            let values = (0..shape.len()).collect::<Vec<_>>();
            let matrix = Matrix::from_row_major(shape, values).unwrap();
            let row_points = [0, rows, rows + 1];
            let column_points = [0, columns, columns + 1];

            for start_row in row_points {
                for end_row in row_points {
                    for start_column in column_points {
                        for end_column in column_points {
                            let result = Region::new(
                                shape,
                                start_row..end_row,
                                start_column..end_column,
                            );

                            match result {
                                Ok(region) => {
                                    let lens = matrix.lens(region).unwrap();
                                    assert_eq!(
                                        lens.shape(),
                                        Shape::new(region.rows(), region.columns()).unwrap()
                                    );
                                    assert_eq!(lens.iter().count(), region.rows() * region.columns());

                                    let expected = (start_row..end_row)
                                        .flat_map(|row| {
                                            (start_column..end_column)
                                                .map(move |column| row * columns + column)
                                        })
                                        .collect::<Vec<_>>();
                                    assert_eq!(lens.iter().copied().collect::<Vec<_>>(), expected);
                                }
                                Err(error) => assert!(matches!(
                                    error,
                                    MatricalError::RegionReversed { .. }
                                        | MatricalError::RegionOutOfBounds { .. }
                                )),
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn lens_types_are_concrete_borrowing_views() {
        fn accept_lens(_: Lens<'_, i32>) {}
        fn accept_lens_mut(_: LensMut<'_, i32>) {}

        let matrix = Matrix::from_row_major(Shape::new(1, 1).unwrap(), vec![7]).unwrap();
        let region = Region::new(matrix.shape(), 0..1, 0..1).unwrap();
        accept_lens(matrix.lens(region).unwrap());

        let mut matrix = Matrix::from_row_major(Shape::new(1, 1).unwrap(), vec![7]).unwrap();
        accept_lens_mut(matrix.lens_mut(region).unwrap());
    }
}
