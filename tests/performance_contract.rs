use matrical::{Index, Matrix, Region, Shape};

fn matrix_4x5() -> Matrix<i32> {
    Matrix::from_row_major(Shape::new(4, 5).unwrap(), (0..20).collect()).unwrap()
}

#[test]
fn optimized_lens_traversal_preserves_full_and_interior_row_major_order() {
    let matrix = matrix_4x5();
    let full = Region::new(matrix.shape(), 0..4, 0..5).unwrap();
    assert_eq!(
        matrix.lens(full).unwrap().iter().copied().collect::<Vec<_>>(),
        (0..20).collect::<Vec<_>>()
    );

    let interior = Region::new(matrix.shape(), 1..4, 1..4).unwrap();
    assert_eq!(
        matrix
            .lens(interior)
            .unwrap()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![6, 7, 8, 11, 12, 13, 16, 17, 18]
    );
}

#[test]
fn optimized_lens_traversal_preserves_single_row_and_column_order() {
    let matrix = matrix_4x5();

    assert_eq!(
        matrix.row(2).unwrap().iter().copied().collect::<Vec<_>>(),
        vec![10, 11, 12, 13, 14]
    );
    assert_eq!(
        matrix.column(3).unwrap().iter().copied().collect::<Vec<_>>(),
        vec![3, 8, 13, 18]
    );
}

#[test]
fn optimized_lens_traversal_preserves_empty_and_zero_dimension_semantics() {
    let matrix = matrix_4x5();
    let empty_rows = Region::new(matrix.shape(), 2..2, 1..4).unwrap();
    let empty_columns = Region::new(matrix.shape(), 1..3, 4..4).unwrap();
    assert_eq!(matrix.lens(empty_rows).unwrap().iter().count(), 0);
    assert_eq!(matrix.lens(empty_columns).unwrap().iter().count(), 0);

    let zero_width = Matrix::<i32>::from_row_major(Shape::new(3, 0).unwrap(), vec![]).unwrap();
    assert_eq!(zero_width.row(1).unwrap().iter().count(), 0);

    let zero_height = Matrix::<i32>::from_row_major(Shape::new(0, 3).unwrap(), vec![]).unwrap();
    assert_eq!(zero_height.column(1).unwrap().iter().count(), 0);

    let zero_by_zero = Matrix::<i32>::from_row_major(Shape::new(0, 0).unwrap(), vec![]).unwrap();
    let empty = Region::new(zero_by_zero.shape(), 0..0, 0..0).unwrap();
    assert_eq!(zero_by_zero.lens(empty).unwrap().iter().count(), 0);
}

#[test]
fn optimized_mutable_traversal_changes_only_selected_cells_in_row_major_order() {
    let mut matrix = matrix_4x5();
    let region = Region::new(matrix.shape(), 1..3, 1..4).unwrap();

    {
        let mut lens = matrix.lens_mut(region).unwrap();
        for (value, replacement) in lens.iter_mut().zip(100..106) {
            *value = replacement;
        }
        assert_eq!(*lens.get(Index::new(0, 0)).unwrap(), 100);
        assert_eq!(*lens.get(Index::new(1, 2)).unwrap(), 105);
        assert_eq!(lens.to_row_major(), vec![100, 101, 102, 103, 104, 105]);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0, 1, 2, 3, 4, 5, 100, 101, 102, 9, 10, 103, 104, 105, 14, 15, 16, 17, 18,
            19,
        ]
    );
}

#[test]
fn foreign_region_still_fails_closed_after_private_view_optimization() {
    let matrix = Matrix::from_row_major(Shape::new(2, 2).unwrap(), vec![1, 2, 3, 4]).unwrap();
    let foreign = Region::new(Shape::new(3, 3).unwrap(), 0..3, 0..3).unwrap();

    assert!(matrix.lens(foreign).is_err());
}
