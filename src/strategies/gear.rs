use crate::{ClampPolicy, Cog, Lens, LensMut, MatricalError, Region, ScalarPolicy, Tag, ValidateCog};

/// The authority/effect class exercised by a Gear execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GearEffect {
    ReadOnly,
    Mutating,
}

/// Inspectable result of a successful Gear execution.
///
/// Failure remains the `Err(MatricalError)` branch; Matrical never fabricates a
/// success report around a failed transformation.
#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionReport<O> {
    gear: &'static str,
    region: Region,
    effect: GearEffect,
    output: O,
    tags: Vec<Tag>,
}

impl<O> ExecutionReport<O> {
    fn new(
        gear: &'static str,
        region: Region,
        effect: GearEffect,
        output: O,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            gear,
            region,
            effect,
            output,
            tags,
        }
    }

    /// Static identity reported by the Gear that ran.
    pub const fn gear(&self) -> &'static str {
        self.gear
    }

    /// Exact caller-selected Region used by the Lens.
    pub const fn region(&self) -> Region {
        self.region
    }

    /// Whether this execution was read-only or mutating.
    pub const fn effect(&self) -> GearEffect {
        self.effect
    }

    /// Borrows the strongly typed Gear output.
    pub const fn output(&self) -> &O {
        &self.output
    }

    /// Consumes the report and returns the strongly typed Gear output.
    pub fn into_output(self) -> O {
        self.output
    }

    /// Provenance Tags in caller-supplied deterministic order.
    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }
}

/// A read-only transformation over exactly the data exposed by an immutable
/// [`Lens`].
///
/// Downstream crates implement this trait directly; no registry, factory, or
/// boxed dynamic object is required. The Lens is the capability boundary: a
/// read Gear receives neither `&Matrix<T>` nor mutable Lens authority.
///
/// A read Gear cannot ask its supplied Lens for mutable access:
///
/// ```compile_fail
/// use matrical::{Index, Lens, MatricalError, ReadGear};
///
/// struct BadGear;
///
/// impl ReadGear<i32> for BadGear {
///     type Context = ();
///     type Output = ();
///
///     fn name(&self) -> &'static str {
///         "bad"
///     }
///
///     fn apply(
///         &self,
///         lens: &Lens<'_, i32>,
///         _context: &Self::Context,
///     ) -> Result<Self::Output, MatricalError> {
///         *lens.get_mut(Index::new(0, 0))? = 9;
///         Ok(())
///     }
/// }
/// ```
pub trait ReadGear<T> {
    type Context: ValidateCog;
    type Output;

    fn name(&self) -> &'static str;

    fn apply(
        &self,
        lens: &Lens<'_, T>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError>;
}

/// A mutating transformation over exactly the data exposed by a [`LensMut`].
///
/// The exclusive mutable Lens is the only normal mutation authority supplied to
/// the Gear. Values outside its selected Region cannot be reached through this
/// contract.
pub trait MutGear<T> {
    type Context: ValidateCog;
    type Output;

    fn name(&self) -> &'static str;

    fn apply(
        &self,
        lens: &mut LensMut<'_, T>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError>;
}

/// Validates typed Cog context, runs a read-only Gear, and returns a report.
pub fn execute_read<T, G>(
    gear: &G,
    lens: &Lens<'_, T>,
    cog: &Cog<G::Context>,
    tags: Vec<Tag>,
) -> Result<ExecutionReport<G::Output>, MatricalError>
where
    G: ReadGear<T>,
{
    let context = cog.context()?;
    context.validate()?;
    let region = lens.region();
    let output = gear.apply(lens, context)?;

    Ok(ExecutionReport::new(
        gear.name(),
        region,
        GearEffect::ReadOnly,
        output,
        tags,
    ))
}

/// Validates typed Cog context, runs a mutating Gear, and returns a report.
pub fn execute_mut<T, G>(
    gear: &G,
    lens: &mut LensMut<'_, T>,
    cog: &Cog<G::Context>,
    tags: Vec<Tag>,
) -> Result<ExecutionReport<G::Output>, MatricalError>
where
    G: MutGear<T>,
{
    let context = cog.context()?;
    context.validate()?;
    let region = lens.region();
    let output = gear.apply(lens, context)?;

    Ok(ExecutionReport::new(
        gear.name(),
        region,
        GearEffect::Mutating,
        output,
        tags,
    ))
}

/// Deterministic read-only sum over an `f64` Lens.
///
/// The natural empty result is `0.0`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SumGear;

impl ReadGear<f64> for SumGear {
    type Context = ();
    type Output = f64;

    fn name(&self) -> &'static str {
        "sum"
    }

    fn apply(
        &self,
        lens: &Lens<'_, f64>,
        _context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        Ok(lens.iter().copied().sum())
    }
}

/// Adds a finite scalar from typed [`ScalarPolicy`] context to every selected
/// element and reports the number of affected elements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddScalarGear;

impl MutGear<f64> for AddScalarGear {
    type Context = ScalarPolicy;
    type Output = usize;

    fn name(&self) -> &'static str {
        "add_scalar"
    }

    fn apply(
        &self,
        lens: &mut LensMut<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        let mut affected = 0;
        for value in lens.iter_mut() {
            *value += context.value();
            affected += 1;
        }
        Ok(affected)
    }
}

/// Multiplies every selected element by a finite scalar from typed
/// [`ScalarPolicy`] context.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScaleGear;

impl MutGear<f64> for ScaleGear {
    type Context = ScalarPolicy;
    type Output = usize;

    fn name(&self) -> &'static str {
        "scale"
    }

    fn apply(
        &self,
        lens: &mut LensMut<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        let mut affected = 0;
        for value in lens.iter_mut() {
            *value *= context.value();
            affected += 1;
        }
        Ok(affected)
    }
}

/// Clamps every selected element to the inclusive validated [`ClampPolicy`]
/// range.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClampGear;

impl MutGear<f64> for ClampGear {
    type Context = ClampPolicy;
    type Output = usize;

    fn name(&self) -> &'static str {
        "clamp"
    }

    fn apply(
        &self,
        lens: &mut LensMut<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        let minimum = context.minimum();
        let maximum = context.maximum();
        let mut affected = 0;

        for value in lens.iter_mut() {
            *value = value.max(minimum).min(maximum);
            affected += 1;
        }

        Ok(affected)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        execute_mut, execute_read, AddScalarGear, ClampGear, GearEffect, ScaleGear, SumGear,
    };
    use crate::{ClampPolicy, Cog, MatricalError, Matrix, Region, ScalarPolicy, Shape, Tag};

    fn matrix() -> Matrix<f64> {
        Matrix::from_row_major(
            Shape::new(2, 3).unwrap(),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        )
        .unwrap()
    }

    #[test]
    fn sum_is_selected_deterministic_and_empty_safe() {
        let matrix = matrix();
        let partial_region = Region::new(matrix.shape(), 0..2, 1..3).unwrap();
        let partial = matrix.lens(partial_region).unwrap();
        let cog = Cog::new(());

        let first = execute_read(&SumGear, &partial, &cog, vec![]).unwrap();
        let second = execute_read(&SumGear, &partial, &cog, vec![]).unwrap();
        assert_eq!(*first.output(), 16.0);
        assert_eq!(first, second);
        assert_eq!(first.effect(), GearEffect::ReadOnly);

        let empty_region = Region::new(matrix.shape(), 1..1, 0..3).unwrap();
        let empty = matrix.lens(empty_region).unwrap();
        assert_eq!(
            *execute_read(&SumGear, &empty, &cog, vec![])
                .unwrap()
                .output(),
            0.0
        );
    }

    #[test]
    fn mutation_changes_only_selected_region() {
        let mut matrix = matrix();
        let region = Region::new(matrix.shape(), 0..2, 1..2).unwrap();
        let tags = vec![Tag::source("unit")];

        {
            let mut lens = matrix.lens_mut(region).unwrap();
            let report = execute_mut(
                &AddScalarGear,
                &mut lens,
                &Cog::new(ScalarPolicy::new(10.0)),
                tags.clone(),
            )
            .unwrap();
            assert_eq!(*report.output(), 2);
            assert_eq!(report.effect(), GearEffect::Mutating);
            assert_eq!(report.region(), region);
            assert_eq!(report.tags(), tags.as_slice());
        }

        assert_eq!(
            matrix.into_row_major(),
            vec![1.0, 12.0, 3.0, 4.0, 15.0, 6.0]
        );
    }

    #[test]
    fn scale_and_clamp_are_distinct_deterministic_mutations() {
        let mut matrix = matrix();
        let region = Region::new(matrix.shape(), 0..2, 0..3).unwrap();

        {
            let mut lens = matrix.lens_mut(region).unwrap();
            execute_mut(
                &ScaleGear,
                &mut lens,
                &Cog::new(ScalarPolicy::new(2.0)),
                vec![],
            )
            .unwrap();
        }
        {
            let mut lens = matrix.lens_mut(region).unwrap();
            execute_mut(
                &ClampGear,
                &mut lens,
                &Cog::new(ClampPolicy::new(4.0, 9.0)),
                vec![],
            )
            .unwrap();
        }

        assert_eq!(
            matrix.into_row_major(),
            vec![4.0, 4.0, 6.0, 8.0, 9.0, 9.0]
        );
    }

    #[test]
    fn empty_mutation_reports_zero_affected_elements() {
        let mut matrix = matrix();
        let region = Region::new(matrix.shape(), 1..1, 0..3).unwrap();
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

    #[test]
    fn missing_and_invalid_context_fail_before_execution() {
        let mut matrix = matrix();
        let region = Region::new(matrix.shape(), 0..1, 0..1).unwrap();

        {
            let mut lens = matrix.lens_mut(region).unwrap();
            assert_eq!(
                execute_mut(
                    &AddScalarGear,
                    &mut lens,
                    &Cog::<ScalarPolicy>::empty(),
                    vec![],
                ),
                Err(MatricalError::InvalidContext)
            );
        }

        {
            let mut lens = matrix.lens_mut(region).unwrap();
            assert_eq!(
                execute_mut(
                    &ClampGear,
                    &mut lens,
                    &Cog::new(ClampPolicy::new(3.0, 2.0)),
                    vec![],
                ),
                Err(MatricalError::InvalidValue)
            );
        }

        assert_eq!(matrix.into_row_major(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }
}
