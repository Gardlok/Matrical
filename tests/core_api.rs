use matrical::{Index, MatricalError, Matrix, Region, Shape};

#[test]
fn downstream_core_api_is_checked_and_deterministic() -> Result<(), MatricalError> {
    let shape = Shape::new(2, 3)?;
    let mut matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4, 5, 6])?;

    assert_eq!(*matrix.get(Index::new(0, 0))?, 1);
    assert_eq!(*matrix.get(Index::new(1, 2))?, 6);

    *matrix.get_mut(Index::new(0, 1))? = 20;
    assert_eq!(
        matrix.iter().copied().collect::<Vec<_>>(),
        vec![1, 20, 3, 4, 5, 6]
    );

    let region = matrix.region(0..2, 1..3)?;
    assert_eq!(region.rows(), 2);
    assert_eq!(region.columns(), 2);
    assert_eq!(matrix.validate_region(region)?, region);

    let independently_checked = Region::new(shape, 1..2, 0..3)?;
    assert_eq!(matrix.validate_region(independently_checked)?, independently_checked);

    assert!(matches!(
        matrix.get(Index::new(2, 0)),
        Err(MatricalError::IndexOutOfBounds)
    ));

    assert!(matches!(
        Matrix::<i32>::from_row_major(shape, vec![1, 2]),
        Err(MatricalError::RowMajorLengthMismatch {
            expected: 6,
            actual: 2,
        })
    ));

    let source: &dyn std::error::Error = &MatricalError::IndexOutOfBounds;
    assert_eq!(source.to_string(), "Index out of bounds");

    Ok(())
}
