use matrical::prelude::*;

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let mut matrix = Matrix::from_row_major(
        shape,
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
        ],
    )?;
    let region = Region::new(shape, 1..3, 1..3)?;
    let tags = vec![
        Tag::source("r5-quickstart"),
        Tag::stage(TagStage::Transform),
        Tag::sequence(1),
    ];

    {
        let lens = matrix.lens(region)?;
        let report = execute_read(&SumGear, &lens, &Cog::new(()), tags.clone())?;
        println!(
            "{} {:?} {:?} -> {}",
            report.gear(),
            report.effect(),
            report.region(),
            report.output()
        );
        assert_eq!(*report.output(), 30.0);
    }

    {
        let mut lens = matrix.lens_mut(region)?;
        let report = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::new(ScalarPolicy::new(10.0)),
            tags,
        )?;
        println!(
            "{} {:?} affected {} values; tags={:?}",
            report.gear(),
            report.effect(),
            report.output(),
            report.tags()
        );
        assert_eq!(*report.output(), 4);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 15.0, 16.0, 7.0, 8.0, 19.0, 20.0, 11.0,
        ]
    );
    Ok(())
}
