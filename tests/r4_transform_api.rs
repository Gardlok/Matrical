use matrical::{
    execute_mut, execute_read, AddScalarGear, ClampGear, ClampPolicy, Cog, GearEffect, Lens,
    MatricalError, Matrix, MutGear, ReadGear, Region, ScalarPolicy, Shape, SumGear, Tag, TagStage,
    ValidateCog,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ThresholdPolicy(f64);

impl ValidateCog for ThresholdPolicy {
    fn validate(&self) -> Result<(), MatricalError> {
        if self.0.is_finite() {
            Ok(())
        } else {
            Err(MatricalError::InvalidValue)
        }
    }
}

struct CountAboveGear;

impl ReadGear<f64> for CountAboveGear {
    type Context = ThresholdPolicy;
    type Output = usize;

    fn name(&self) -> &'static str {
        "downstream_count_above"
    }

    fn apply(
        &self,
        lens: &Lens<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        Ok(lens.iter().filter(|value| **value > context.0).count())
    }
}

fn matrix() -> Matrix<f64> {
    Matrix::from_row_major(
        Shape::new(3, 4).unwrap(),
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
        ],
    )
    .unwrap()
}

#[test]
fn public_api_composes_lens_gear_cog_tag_and_report() {
    let mut matrix = matrix();
    let region = Region::new(matrix.shape(), 1..3, 1..3).unwrap();
    let provenance = vec![
        Tag::source("external-fixture"),
        Tag::stage(TagStage::Transform),
        Tag::sequence(42),
    ];

    {
        let lens = matrix.lens(region).unwrap();
        let report = execute_read(&SumGear, &lens, &Cog::new(()), provenance.clone()).unwrap();

        assert_eq!(report.gear(), "sum");
        assert_eq!(report.region(), region);
        assert_eq!(report.effect(), GearEffect::ReadOnly);
        assert_eq!(*report.output(), 30.0);
        assert_eq!(report.tags(), provenance.as_slice());
    }

    {
        let mut lens = matrix.lens_mut(region).unwrap();
        let report = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::new(ScalarPolicy::new(10.0)),
            provenance.clone(),
        )
        .unwrap();

        assert_eq!(report.gear(), "add_scalar");
        assert_eq!(report.region(), region);
        assert_eq!(report.effect(), GearEffect::Mutating);
        assert_eq!(*report.output(), 4);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 15.0, 16.0, 7.0, 8.0, 19.0, 20.0, 11.0,
        ]
    );
}

#[test]
fn downstream_defined_static_gear_needs_no_registry_or_private_api() {
    let matrix = matrix();
    let region = Region::new(matrix.shape(), 0..3, 0..4).unwrap();
    let lens = matrix.lens(region).unwrap();

    let report = execute_read(
        &CountAboveGear,
        &lens,
        &Cog::new(ThresholdPolicy(7.0)),
        vec![Tag::source("downstream")],
    )
    .unwrap();

    assert_eq!(report.gear(), "downstream_count_above");
    assert_eq!(*report.output(), 4);
}

#[test]
fn missing_and_invalid_typed_context_are_public_failures() {
    let mut matrix = matrix();
    let region = Region::new(matrix.shape(), 0..1, 0..2).unwrap();

    {
        let mut lens = matrix.lens_mut(region).unwrap();
        let missing = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::<ScalarPolicy>::empty(),
            vec![],
        );
        assert_eq!(missing, Err(MatricalError::InvalidContext));
    }

    {
        let mut lens = matrix.lens_mut(region).unwrap();
        let invalid = execute_mut(
            &ClampGear,
            &mut lens,
            &Cog::new(ClampPolicy::new(8.0, 2.0)),
            vec![],
        );
        assert_eq!(invalid, Err(MatricalError::InvalidValue));
    }
}

#[test]
fn tags_are_report_metadata_not_an_execution_channel() {
    let matrix = matrix();
    let region = Region::new(matrix.shape(), 0..2, 0..2).unwrap();
    let lens = matrix.lens(region).unwrap();
    let cog = Cog::new(());

    let first = execute_read(
        &SumGear,
        &lens,
        &cog,
        vec![Tag::source("alpha"), Tag::sequence(1)],
    )
    .unwrap();
    let second = execute_read(
        &SumGear,
        &lens,
        &cog,
        vec![Tag::source("rm -rf /"), Tag::sequence(999)],
    )
    .unwrap();

    assert_eq!(first.output(), second.output());
    assert_ne!(first.tags(), second.tags());
}

#[test]
fn empty_selection_has_natural_read_and_mutating_results() {
    let mut matrix = matrix();
    let region = Region::new(matrix.shape(), 2..2, 0..4).unwrap();

    {
        let lens = matrix.lens(region).unwrap();
        let report = execute_read(&SumGear, &lens, &Cog::new(()), vec![]).unwrap();
        assert_eq!(*report.output(), 0.0);
    }

    {
        let mut lens = matrix.lens_mut(region).unwrap();
        let report = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::new(ScalarPolicy::new(1.0)),
            vec![],
        )
        .unwrap();
        assert_eq!(*report.output(), 0);
    }
}
