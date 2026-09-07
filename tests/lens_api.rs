use matrical::{Index, MatricalError, Matrix, Region, Shape};

#[test]
fn public_lens_api_borrows_selects_and_reports_typed_failures() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let matrix = Matrix::from_row_major(shape, (0..12).collect())?;
    let region = Region::new(shape, 1..3, 1..4)?;
    let lens = matrix.lens(region)?;

    assert_eq!(lens.shape(), Shape::new(2, 3)?);
    assert_eq!(*lens.get(Index::new(0, 0))?, 5);
    assert_eq!(*lens.get(Index::new(1, 2))?, 11);
    assert_eq!(lens.iter().copied().collect::<Vec<_>>(), vec![5, 6, 7, 9, 10, 11]);

    let first_row = matrix.row(0)?;
    assert_eq!(first_row.to_row_major(), vec![0, 1, 2, 3]);

    let last_column = matrix.column(3)?;
    assert_eq!(last_column.to_row_major(), vec![3, 7, 11]);

    let foreign = Region::new(Shape::new(4, 5)?, 0..4, 0..5)?;
    assert!(matches!(
        matrix.lens(foreign),
        Err(MatricalError::RegionOutOfBounds { .. })
    ));
    assert!(matches!(
        lens.get(Index::new(2, 0)),
        Err(MatricalError::IndexOutOfBounds)
    ));

    Ok(())
}

#[test]
fn public_mutable_lens_updates_the_parent_matrix() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let mut matrix = Matrix::from_row_major(shape, (0..12).collect())?;
    let region = Region::new(shape, 1..3, 1..3)?;

    {
        let mut lens = matrix.lens_mut(region)?;
        *lens.get_mut(Index::new(0, 0))? = 50;
        for value in lens.iter_mut() {
            *value += 100;
        }
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![0, 1, 2, 3, 4, 150, 106, 7, 8, 109, 110, 11]
    );
    Ok(())
}

#[test]
fn public_empty_row_and_column_semantics_are_explicit() -> Result<(), MatricalError> {
    let zero_rows = Matrix::<i32>::from_row_major(Shape::new(0, 3)?, vec![])?;
    let column = zero_rows.column(1)?;
    assert_eq!(column.shape(), Shape::new(0, 1)?);
    assert!(column.is_empty());

    let zero_columns = Matrix::<i32>::from_row_major(Shape::new(2, 0)?, vec![])?;
    let row = zero_columns.row(1)?;
    assert_eq!(row.shape(), Shape::new(1, 0)?);
    assert!(row.is_empty());

    assert!(matches!(
        zero_rows.row(0),
        Err(MatricalError::IndexOutOfBounds)
    ));
    assert!(matches!(
        zero_columns.column(0),
        Err(MatricalError::IndexOutOfBounds)
    ));
    Ok(())
}
