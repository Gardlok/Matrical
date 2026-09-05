use matrical::{Matrix, MatrixSnapshot, Shape, DENSE_SNAPSHOT_VERSION};

#[test]
fn borrowed_snapshot_clones_values_and_exposes_inert_metadata() {
    let shape = Shape::new(2, 3).unwrap();
    let matrix = Matrix::from_row_major(shape, vec![1_i64, 2, 3, 4, 5, 6]).unwrap();

    let snapshot = matrix.snapshot();

    assert_eq!(snapshot.version(), DENSE_SNAPSHOT_VERSION);
    assert_eq!(snapshot.rows(), 2);
    assert_eq!(snapshot.columns(), 3);
    assert_eq!(snapshot.len(), 6);
    assert!(!snapshot.is_empty());
    assert_eq!(snapshot.row_major(), &[1, 2, 3, 4, 5, 6]);

    // The borrowed path cloned values; the original Matrix remains usable.
    assert_eq!(matrix.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn snapshot_reconstructs_through_checked_matrix_invariants() {
    let shape = Shape::new(2, 2).unwrap();
    let matrix = Matrix::from_row_major(shape, vec![10_i64, 20, 30, 40]).unwrap();

    let reconstructed = matrix.snapshot().into_matrix().unwrap();

    assert_eq!(reconstructed.shape(), shape);
    assert_eq!(reconstructed.into_row_major(), vec![10, 20, 30, 40]);
}

#[test]
fn consuming_snapshot_roundtrip_does_not_require_clone() {
    #[derive(Debug, PartialEq, Eq)]
    struct NonClone(i32);

    let shape = Shape::new(1, 3).unwrap();
    let matrix = Matrix::from_row_major(
        shape,
        vec![NonClone(7), NonClone(8), NonClone(9)],
    )
    .unwrap();

    let snapshot = matrix.into_snapshot();
    assert_eq!(snapshot.version(), DENSE_SNAPSHOT_VERSION);
    assert_eq!(snapshot.rows(), 1);
    assert_eq!(snapshot.columns(), 3);

    let reconstructed = snapshot.into_matrix().unwrap();
    assert_eq!(
        reconstructed.into_row_major(),
        vec![NonClone(7), NonClone(8), NonClone(9)]
    );
}

#[test]
fn zero_dimension_matrices_roundtrip_without_values() {
    for shape in [
        Shape::new(0, 0).unwrap(),
        Shape::new(0, 5).unwrap(),
        Shape::new(5, 0).unwrap(),
    ] {
        let matrix = Matrix::<i32>::from_row_major(shape, vec![]).unwrap();
        let snapshot = matrix.into_snapshot();

        assert_eq!(snapshot.rows(), shape.rows() as u64);
        assert_eq!(snapshot.columns(), shape.columns() as u64);
        assert_eq!(snapshot.len(), 0);
        assert!(snapshot.is_empty());

        let reconstructed = snapshot.into_matrix().unwrap();
        assert_eq!(reconstructed.shape(), shape);
        assert!(reconstructed.is_empty());
    }
}

#[test]
fn snapshot_can_be_reconstructed_via_try_from() {
    let shape = Shape::new(1, 2).unwrap();
    let snapshot: MatrixSnapshot<_> =
        Matrix::from_row_major(shape, vec![3_i64, 4]).unwrap().into_snapshot();

    let matrix = Matrix::try_from(snapshot).unwrap();
    assert_eq!(matrix.into_row_major(), vec![3, 4]);
}
