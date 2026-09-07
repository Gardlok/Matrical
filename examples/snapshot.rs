use matrical::{MatricalError, Matrix, MatrixSnapshot, Shape};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shape = Shape::new(2, 3)?;
    let matrix = Matrix::from_row_major(shape, vec![1_i64, 2, 3, 4, 5, 6])?;

    let snapshot = matrix.into_snapshot();
    let encoded = serde_json::to_string(&snapshot)?;
    let decoded: MatrixSnapshot<i64> = serde_json::from_str(&encoded)?;
    let reconstructed = decoded.into_matrix()?;

    if reconstructed.into_row_major() != vec![1, 2, 3, 4, 5, 6] {
        return Err(Box::new(MatricalError::ShouldNotOccur));
    }

    println!("{encoded}");
    Ok(())
}
