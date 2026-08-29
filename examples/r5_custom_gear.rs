use matrical::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Threshold {
    minimum: f64,
}

impl ValidateCog for Threshold {
    fn validate(&self) -> Result<(), MatricalError> {
        if self.minimum.is_finite() {
            Ok(())
        } else {
            Err(MatricalError::InvalidValue)
        }
    }
}

struct CountAtLeast;

impl ReadGear<f64> for CountAtLeast {
    type Context = Threshold;
    type Output = usize;

    fn name(&self) -> &'static str {
        "count_at_least"
    }

    fn apply(
        &self,
        lens: &Lens<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        Ok(lens
            .iter()
            .filter(|value| **value >= context.minimum)
            .count())
    }
}

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(2, 3)?;
    let matrix = Matrix::from_row_major(shape, vec![1.0, 5.0, 9.0, 3.0, 7.0, 11.0])?;
    let region = Region::new(shape, 0..2, 1..3)?;
    let lens = matrix.lens(region)?;
    let cog = Cog::new(Threshold { minimum: 8.0 });

    let report = execute_read(
        &CountAtLeast,
        &lens,
        &cog,
        vec![Tag::source("downstream-example")],
    )?;
    let count: usize = *report.output();

    assert_eq!(count, 2);
    assert_eq!(report.gear(), "count_at_least");
    Ok(())
}
