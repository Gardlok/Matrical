use matrical::{Index, Matrix, Shape};

fn main() -> Result<(), matrical::MatricalError> {
    let shape = Shape::new(2, 3)?;
    let mut matrix = Matrix::from_row_major(shape, vec![1, 2, 3, 4, 5, 6])?;

    *matrix.get_mut(Index::new(1, 1))? = 50;
    let region = matrix.region(0..2, 1..3)?;

    println!(
        "shape={}x{}, region={}x{}, row-major={:?}",
        matrix.rows(),
        matrix.columns(),
        region.rows(),
        region.columns(),
        matrix.iter().collect::<Vec<_>>()
    );

    Ok(())
}
