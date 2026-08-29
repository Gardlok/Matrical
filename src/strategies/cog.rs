use crate::MatricalError;

/// Validation contract for typed Gear context and policy.
///
/// Context validation is deliberately small and static: callers provide a
/// concrete Rust type, and execution validates that type before a Gear receives
/// it. No registry, string lookup, or `Any` downcast is involved.
pub trait ValidateCog {
    /// Validates context before Gear execution.
    fn validate(&self) -> Result<(), MatricalError>;
}

impl ValidateCog for () {
    fn validate(&self) -> Result<(), MatricalError> {
        Ok(())
    }
}

/// A typed optional context container for Gear execution.
///
/// `Cog<C>` preserves the concrete Rust type `C`. A missing required context is
/// returned as [`MatricalError::InvalidContext`] by [`Cog::context`] and by the
/// central Gear execution functions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cog<C> {
    context: Option<C>,
}

impl<C> Cog<C> {
    /// Creates a Cog containing required context.
    pub const fn new(context: C) -> Self {
        Self {
            context: Some(context),
        }
    }

    /// Creates a Cog with no context.
    ///
    /// Passing this Cog to a Gear that requires `C` produces
    /// [`MatricalError::InvalidContext`].
    pub const fn empty() -> Self {
        Self { context: None }
    }

    /// Creates a Cog from an already optional context value.
    pub const fn from_option(context: Option<C>) -> Self {
        Self { context }
    }

    /// Reports whether typed context is present without resolving it.
    pub const fn is_present(&self) -> bool {
        self.context.is_some()
    }

    /// Borrows the typed context or returns a typed missing-context failure.
    pub fn context(&self) -> Result<&C, MatricalError> {
        self.context
            .as_ref()
            .ok_or(MatricalError::InvalidContext)
    }

    /// Consumes the Cog and returns its optional typed context.
    pub fn into_option(self) -> Option<C> {
        self.context
    }
}

/// A finite scalar policy used by built-in scalar transformation Gears.
///
/// Construction itself is infallible so callers can compose policy values
/// naturally; [`ValidateCog::validate`] rejects non-finite values before a Gear
/// executes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalarPolicy {
    value: f64,
}

impl ScalarPolicy {
    /// Creates a scalar policy. Non-finite values fail during Cog validation.
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    /// Returns the configured scalar.
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl ValidateCog for ScalarPolicy {
    fn validate(&self) -> Result<(), MatricalError> {
        if self.value.is_finite() {
            Ok(())
        } else {
            Err(MatricalError::InvalidValue)
        }
    }
}

/// Inclusive lower/upper bounds used by [`crate::ClampGear`].
///
/// Bounds are validated immediately before Gear execution. Both must be finite
/// and `minimum <= maximum`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClampPolicy {
    minimum: f64,
    maximum: f64,
}

impl ClampPolicy {
    /// Creates clamp bounds. Invalid ordering or non-finite values fail during
    /// Cog validation rather than at construction.
    pub const fn new(minimum: f64, maximum: f64) -> Self {
        Self { minimum, maximum }
    }

    /// Returns the inclusive lower bound.
    pub const fn minimum(self) -> f64 {
        self.minimum
    }

    /// Returns the inclusive upper bound.
    pub const fn maximum(self) -> f64 {
        self.maximum
    }
}

impl ValidateCog for ClampPolicy {
    fn validate(&self) -> Result<(), MatricalError> {
        if !self.minimum.is_finite()
            || !self.maximum.is_finite()
            || self.minimum > self.maximum
        {
            return Err(MatricalError::InvalidValue);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ClampPolicy, Cog, ScalarPolicy, ValidateCog};
    use crate::MatricalError;

    #[test]
    fn typed_context_is_retrievable_without_downcast() {
        let cog = Cog::new(ScalarPolicy::new(2.5));

        assert_eq!(cog.context().unwrap().value(), 2.5);
    }

    #[test]
    fn missing_context_is_typed_failure() {
        let cog = Cog::<ScalarPolicy>::empty();

        assert_eq!(cog.context(), Err(MatricalError::InvalidContext));
    }

    #[test]
    fn invalid_context_is_rejected_by_validation() {
        let policy = ClampPolicy::new(5.0, 1.0);

        assert_eq!(policy.validate(), Err(MatricalError::InvalidValue));
    }
}
