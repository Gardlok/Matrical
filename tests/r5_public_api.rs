use matrical::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct MinimumPolicy(f64);

impl ValidateCog for MinimumPolicy {
    fn validate(&self) -> Result<(), MatricalError> {
        if self.0.is_finite() {
            Ok(())
        } else {
            Err(MatricalError::InvalidValue)
        }
    }
}

struct CountAbove;

impl ReadGear<f64> for CountAbove {
    type Context = MinimumPolicy;
    type Output = usize;

    fn name(&self) -> &'static str {
        "r5_count_above"
    }

    fn apply(
        &self,
        lens: &Lens<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        Ok(lens.iter().filter(|value| **value > context.0).count())
    }
}

#[test]
fn recommended_prelude_supports_the_complete_learning_flow() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let mut matrix = Matrix::from_row_major(
        shape,
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
        ],
    )?;
    assert_eq!(*matrix.get(Index::new(1, 2))?, 6.0);

    let region = Region::new(shape, 1..3, 1..3)?;
    let tags = vec![
        Tag::source("r5-public-api"),
        Tag::stage(TagStage::Transform),
        Tag::sequence(5),
    ];

    {
        let lens = matrix.lens(region)?;
        assert_eq!(lens.get(Index::new(0, 0))?, &5.0);

        let sum = execute_read(&SumGear, &lens, &Cog::new(()), tags.clone())?;
        assert_eq!(*sum.output(), 30.0);
        assert_eq!(sum.effect(), GearEffect::ReadOnly);
        assert_eq!(sum.tags(), tags.as_slice());

        let custom = execute_read(
            &CountAbove,
            &lens,
            &Cog::new(MinimumPolicy(7.0)),
            vec![],
        )?;
        assert_eq!(*custom.output(), 2);
    }

    {
        let mut lens = matrix.lens_mut(region)?;
        let report = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::new(ScalarPolicy::new(10.0)),
            tags,
        )?;
        assert_eq!(*report.output(), 4);
        assert_eq!(report.effect(), GearEffect::Mutating);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 15.0, 16.0, 7.0, 8.0, 19.0, 20.0, 11.0,
        ]
    );

    let mismatch = Matrix::from_row_major(Shape::new(1, 2)?, vec![1.0]);
    assert!(matches!(
        mismatch,
        Err(MatricalError::RowMajorLengthMismatch {
            expected: 2,
            actual: 1,
        })
    ));

    Ok(())
}
