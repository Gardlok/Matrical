use matrical::{Index, MatricalError, Matrix, Region, Shape};

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let mut matrix = Matrix::from_row_major(shape, (0..12).collect())?;
    let region = Region::new(shape, 1..3, 1..4)?;

    {
        let lens = matrix.lens(region)?;
        println!("selected shape: {} x {}", lens.rows(), lens.columns());
        println!("top-left selected value: {}", lens.get(Index::new(0, 0))?);
        println!("selected row-major values: {:?}", lens.to_row_major());
    }

    {
        let mut lens = matrix.lens_mut(region)?;
        for value in lens.iter_mut() {
            *value += 100;
        }
    }

    println!("parent after LensMut: {:?}", matrix.iter().collect::<Vec<_>>());
    println!("first row: {:?}", matrix.row(0)?.to_row_major());
    println!("last column: {:?}", matrix.column(3)?.to_row_major());

    Ok(())
}
